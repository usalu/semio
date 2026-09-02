//! 🏭️ Writes the `mathematical@1/any` JSON-carrier fixture corpus. Every pair is checked for OBSERVABILITY
//! before it is written — a mutation whose projection does not move is refused, not committed.
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use mathematical_json::{apply, arrange, build_seed, project, KINDS};

fn main() {
    let out_root = std::env::args().nth(1).unwrap_or_else(|| "../../🧫️fixtures".to_string());
    let seed = build_seed();
    let mut failures: Vec<String> = Vec::new();
    let mut written = 0usize;
    for kind in KINDS {
        let base = arrange(kind, &seed);
        let mutated = match apply(kind, &base) {
            Ok(value) => value,
            Err(error) => {
                failures.push(format!("{kind}: apply: {error}"));
                continue;
            }
        };
        let base_bytes = format!("{}\n", serde_json::to_string_pretty(&base).expect("seed serialises"));
        let mutated_bytes = format!("{}\n", serde_json::to_string_pretty(&mutated).expect("mutation serialises"));
        match (project(base_bytes.as_bytes()), project(mutated_bytes.as_bytes())) {
            (Ok(before), Ok(after)) => assert_ne!(before, after, "{kind}: every fixture pair must be observable in the carrier projection"),
            (Err(error), _) | (_, Err(error)) => {
                failures.push(format!("{kind}: project: {error}"));
                continue;
            }
        }
        let dir = PathBuf::from(&out_root).join(kind);
        fs::create_dir_all(&dir).expect("mkdir");
        fs::write(dir.join("before.json"), &base_bytes).expect("write before.json");
        fs::write(dir.join("after.json"), &mutated_bytes).expect("write after.json");
        written += 1;
        println!("{kind}: observable before={}B after={}B", base_bytes.len(), mutated_bytes.len());
    }
    println!("[generate] wrote {written}/{} fixture pair(s) into {out_root}", KINDS.len());
    if !failures.is_empty() {
        for failure in &failures {
            eprintln!("[generate] {failure}");
        }
        exit(1);
    }
}
