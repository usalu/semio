//! 🔍️ The READER: projects a committed equation JSON carrier through `json` (json-rust) and compares
//! two of them. Nothing here applies a mutation or predicts what one should produce.
//!
//! usage: reader project <file.json> | reader compare <expected.json> <actual.json>
use std::process::exit;

use equation_json::project;
use json::{object, JsonValue};

fn report(probe: &str, status: &str, measurements: JsonValue) -> String {
    object! {
        "schema": "semio.repository-test.probe-report/v2",
        "probe": probe,
        "probeVersion": "json@0.12",
        "engine": {"family": "json-rust", "implementation": "json-rust 0.12 value tree", "version": "0.12"},
        "status": status,
        "durationMs": 0,
        "measurements": measurements,
    }
    .dump()
}

fn read(path: &str) -> Result<JsonValue, String> {
    project(&std::fs::read(path).map_err(|error| error.to_string())?)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("project") => match read(args.get(2).expect("usage: reader project <file.json>")) {
            Ok(value) => println!("{}", report("equation-json-project", "ok", value)),
            Err(error) => {
                println!("{}", report("equation-json-project", "failed", object! {"error": error}));
                exit(1);
            }
        },
        Some("compare") => {
            let expected = args.get(2).expect("usage: reader compare <expected> <actual>");
            let actual = args.get(3).expect("usage: reader compare <expected> <actual>");
            match (read(expected), read(actual)) {
                (Ok(left), Ok(right)) => println!("{}", report("equation-json-compare", "ok", object! {"equal": left == right, "expected": left, "actual": right})),
                (Err(error), _) | (_, Err(error)) => {
                    println!("{}", report("equation-json-compare", "failed", object! {"error": error}));
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: reader project <file.json> | reader compare <expected.json> <actual.json>");
            exit(2);
        }
    }
}
