//! ⏪️ before it is written — a mutation whose projection does not move is refused, not committed.
//! 🏭️ Writes the `fem2d@1/any` JSON-carrier fixture corpus. Every pair is checked for OBSERVABILITY
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use fem2d_json::apply;

use fem2d_json::arrange;

use fem2d_json::build_seed;

use fem2d_json::project;

use fem2d_json::render;

use fem2d_json::KINDS;

fn fixture_directory(kind: &str) -> Option<&'static str> {
    match kind {
        "create-region" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🗺️create-region"),
        "delete-region" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🚫️delete-region"),
        "replace-region" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🔄️replace-region"),
        "move-selection" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🧭️move-selection"),
        "change-combination-name" => Some("🏋️load/🧪️tests/🧫️fixtures/🩹️change-combination-name"),
        "change-combination-term" => Some("🏋️load/🧪️tests/🧫️fixtures/🩹️change-combination-term"),
        "create-node" => Some("🕸️mesh/🧪️tests/🧫️fixtures/⚪️create-node"),
        "delete-node" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🕳️delete-node"),
        "create-element" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🧩️create-element"),
        "delete-element" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🗑️delete-element"),
        "replace-element" => Some("🕸️mesh/🧪️tests/🧫️fixtures/♻️replace-element"),
        "create-material" => Some("🧱️material/🧪️tests/🧫️fixtures/🌱️create-material"),
        "delete-material" => Some("🧱️material/🧪️tests/🧫️fixtures/🗑️delete-material"),
        "replace-material" => Some("🧱️material/🧪️tests/🧫️fixtures/🔁️replace-material"),
        "change-support" => Some("🛡️boundary/🧪️tests/🧫️fixtures/🩹️change-support"),
        "change-element" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🩹️change-element"),
        "change-load" => Some("🏋️load/🧪️tests/🧫️fixtures/🩹️change-load"),
        "change-region" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🩹️change-region"),
        "change-node" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🩹️change-node"),
        "change-section" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🩹️change-section"),
        "change-material" => Some("🧱️material/🧪️tests/🧫️fixtures/🩹️change-material"),
        "create-section" => Some("🕸️mesh/🧪️tests/🧫️fixtures/📐️create-section"),
        "delete-section" => Some("🕸️mesh/🧪️tests/🧫️fixtures/✂️delete-section"),
        "replace-section" => Some("🕸️mesh/🧪️tests/🧫️fixtures/📏️replace-section"),
        "create-support" => Some("🛡️boundary/🧪️tests/🧫️fixtures/🛡️create-support"),
        "delete-support" => Some("🛡️boundary/🧪️tests/🧫️fixtures/🗑️delete-support"),
        "replace-support" => Some("🛡️boundary/🧪️tests/🧫️fixtures/🔁️replace-support"),
        "create-load-case" => Some("🏋️load/🧪️tests/🧫️fixtures/📋️create-load-case"),
        "delete-load-case" => Some("🏋️load/🧪️tests/🧫️fixtures/🗑️delete-load-case"),
        "add-load" => Some("🏋️load/🧪️tests/🧫️fixtures/➕️add-load"),
        "remove-load" => Some("🏋️load/🧪️tests/🧫️fixtures/➖️remove-load"),
        "change-load-case-self-weight" => Some("🏋️load/🧪️tests/🧫️fixtures/⚖️change-load-case-self-weight"),
        "create-combination" => Some("🏋️load/🧪️tests/🧫️fixtures/🔗️create-combination"),
        "delete-combination" => Some("🏋️load/🧪️tests/🧫️fixtures/✂️delete-combination"),
        "update-analysis-settings" => Some("📈️analysis/🧪️tests/🧫️fixtures/🎛️update-analysis-settings"),
        "replace-node" => Some("🕸️mesh/🧪️tests/🧫️fixtures/🔁️replace-node"),
        "replace-load" => Some("🏋️load/🧪️tests/🧫️fixtures/🔁️replace-load"),
        "change-load-case-name" => Some("🏋️load/🧪️tests/🧫️fixtures/🏷️change-load-case-name"),
        "replace-combination" => Some("🏋️load/🧪️tests/🧫️fixtures/🔁️replace-combination"),
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
        let base_bytes = render(&base);
        let mutated_bytes = render(&mutated);
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

/// 🧪️ Every declared semantic event has one registered carrier destination.
#[test]
fn every_semantic_kind_has_a_unique_fixture_destination(){let routes:std::collections::BTreeSet<_>=KINDS.iter().map(|kind|fixture_directory(kind).expect("declared semantic event requires a carrier destination")).collect();assert_eq!(routes.len(),KINDS.len());assert_eq!(KINDS.len(),39);}