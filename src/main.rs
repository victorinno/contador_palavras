use std::{collections::HashMap, path::PathBuf};

use glob::glob_with;
use glob::MatchOptions;
use pdf_extract::extract_text;
use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(default_value = ".")]
    folder: String,
    #[structopt(default_value = "./results.csv")]
    out_folder: String,
    #[structopt(short = "f", default_value = "pdf")]
    format: String,
    #[structopt(short = "q", default_value = "")]
    query: String,
}

fn count_words(text: String) -> HashMap<String, u128> {
    let result: HashMap<String, u128> = HashMap::new();
    let iter = text.split_whitespace();
    iter.fold(result, |mut map, word| {
        let counter = map.entry(word.to_string()).or_insert(0);
        *counter += 1;
        map
    })
}

fn read_pdf(path: &PathBuf) -> HashMap<String, u128> {
    let result = extract_text(path);
    match result {
        Ok(text) => count_words(text),
        _ => HashMap::new(),
    }
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut results: HashMap<String, HashMap<String, u128>> = HashMap::new();

    for entry in glob_with(&(args.folder + "/**/*." + &args.format), options).unwrap() {
        if let Ok(path) = entry {
            if &args.format == "pdf" {
                let counts = read_pdf(&path);
                results.insert(path.display().to_string(), counts);
            }
        }
    }

    println!("{:?}", results);
    let file_path = PathBuf::from(args.out_folder);
    let mut wtr = csv::Writer::from_path(file_path)?;

    wtr.write_record(&["file", "word", "count"])?;
    for k in results.keys() {
        let counts = results.get(k);
        for ck in counts.expect("msg").keys() {
            wtr.write_record(&[k, ck, &counts.expect("").get(ck).expect("msg").to_string()])?;
        }
    }

    wtr.flush()?;
    Ok(())
}
