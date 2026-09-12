//! 🏭️ Writes the `fem3d@1/any` JSON-carrier fixture corpus. Every pair is checked for OBSERVABILITY
//! before it is written — a mutation whose projection does not move is refused, not committed.
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use fem3d_json::{apply, arrange, build_seed, project, KINDS};

fn fixture_directory(kind: &str) -> Option<&'static str> {
    match kind {
        "create-node" => Some("🕸️mesh/🧫️fixtures/⚪️create-node"),
        "delete-node" => Some("🕸️mesh/🧫️fixtures/🕳️delete-node"),
        "create-element" => Some("🕸️mesh/🧫️fixtures/🧩️create-element"),
        "delete-element" => Some("🕸️mesh/🧫️fixtures/🗑️delete-element"),
        "replace-element" => Some("🕸️mesh/🧫️fixtures/♻️replace-element"),
        "create-material" => Some("🧱️material/🧫️fixtures/🌱️create-material"),
        "delete-material" => Some("🧱️material/🧫️fixtures/🗑️delete-material"),
        "replace-material" => Some("🧱️material/🧫️fixtures/🔁️replace-material"),
        "create-section" => Some("🕸️mesh/🧫️fixtures/📐️create-section"),
        "delete-section" => Some("🕸️mesh/🧫️fixtures/✂️delete-section"),
        "replace-section" => Some("🕸️mesh/🧫️fixtures/📏️replace-section"),
        "create-support" => Some("🛡️boundary/🧫️fixtures/🛡️create-support"),
        "delete-support" => Some("🛡️boundary/🧫️fixtures/🗑️delete-support"),
        "replace-support" => Some("🛡️boundary/🧫️fixtures/🔁️replace-support"),
        "create-load-case" => Some("🏋️load/🧫️fixtures/📋️create-load-case"),
        "delete-load-case" => Some("🏋️load/🧫️fixtures/🗑️delete-load-case"),
        "add-load" => Some("🏋️load/🧫️fixtures/➕️add-load"),
        "remove-load" => Some("🏋️load/🧫️fixtures/➖️remove-load"),
        "change-load-case-self-weight" => Some("🏋️load/🧫️fixtures/⚖️change-load-case-self-weight"),
        "create-combination" => Some("🏋️load/🧫️fixtures/🔗️create-combination"),
        "delete-combination" => Some("🏋️load/🧫️fixtures/✂️delete-combination"),
        "update-analysis-settings" => Some("📈️analysis/🧫️fixtures/🎛️update-analysis-settings"),
        _ => None,
    }
}

fn main() {
    let out_root = std::env::args().nth(1).unwrap_or_else(|| "../..".to_string());
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
        let base_bytes = format!("{}\n", pack::json::to_string(&base));
        let mutated_bytes = format!("{}\n", pack::json::to_string(&mutated));
        match (project(base_bytes.as_bytes()), project(mutated_bytes.as_bytes())) {
            (Ok(before), Ok(after)) => {
                if before == after {
                    failures.push(format!("{kind}: not observable in the carrier projection"));
                    continue;
                }
            }
            (Err(error), _) | (_, Err(error)) => {
                failures.push(format!("{kind}: project: {error}"));
                continue;
            }
        }
        let Some(relative_directory) = fixture_directory(kind) else {
            failures.push(format!("{kind}: no semantic fixture directory is registered"));
            continue;
        };
        let dir = PathBuf::from(&out_root).join(relative_directory);
        fs::create_dir_all(&dir).expect("mkdir");
        fs::write(dir.join("⏮️before.json"), &base_bytes).expect("write ⏮️before.json");
        fs::write(dir.join("⏭️after.json"), &mutated_bytes).expect("write ⏭️after.json");
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
