//! 🧨️ Ticket tool of work package K: holds the Rust validator to the findings `fuzz_validation.ts` recorded from the TypeScript validator for a few thousand broken copies of the sample documents.
//!
//! Mounted by the scratch crate only (`#[cfg(test)] #[path] mod findings;`), never by the product crate. A mutant
//! the typed twin decodes must yield exactly the TypeScript findings (pointer and code, in order). A mutant the twin
//! refuses must carry at least one finding of the type-level structure in TypeScript (`type-invalid`, `required`,
//! `property-unknown`, `value-invalid`, or `length-invalid` of a tuple) — the documented division of labour between
//! serde and the validator. From the repository root:
//!
//!   bun <ticket>/fuzz_validation.ts
//!   bash <ticket>/rust_scratch.sh wp-k test --offline findings -- --nocapture

use crate::schema::tests::{entries, json};
use crate::schema::{Ensemble, Menagerie, Species};
use crate::validation::{ensemble_issues, menagerie_issues, species_issues};
use serde_json::Value;
use std::path::PathBuf;

#[test]
fn the_rust_validator_reports_the_findings_of_the_typescript_twin_for_every_mutant() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../validation-mutants.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error} — run fuzz_validation.ts first", path.display()));
    let document: Value = serde_json::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    let (mut decoded, mut refused, mut valid, mut findings) = (0usize, 0usize, 0usize, 0usize);
    let mut disagreements = Vec::new();
    for mutant in entries(&document["mutants"]) {
        let broken = &mutant["document"];
        let issues = match mutant["definition"].as_str() {
            Some("Species") => serde_json::from_value::<Species>(broken.clone()).ok().map(|species| species_issues(&species)),
            Some("Menagerie") => serde_json::from_value::<Menagerie>(broken.clone()).ok().map(|menagerie| menagerie_issues(&menagerie)),
            Some("Ensemble") => serde_json::from_value::<Ensemble>(broken.clone()).ok().map(|ensemble| ensemble_issues(&ensemble)),
            other => panic!("unknown definition {other:?}"),
        };
        match issues {
            Some(issues) => {
                decoded += 1;
                valid += usize::from(issues.is_empty());
                findings += issues.len();
                if json(&issues) != mutant["issues"] {
                    disagreements.push(format!("mutant {} {}: rust {} ≠ typescript {}", mutant["index"], mutant["applied"], json(&issues), mutant["issues"]));
                }
            }
            None => {
                refused += 1;
                let structural = entries(&mutant["issues"]).iter().any(|issue| {
                    let (code, path) = (issue["code"].as_str().unwrap_or_default(), issue["path"].as_str().unwrap_or_default());
                    ["type-invalid", "required", "property-unknown", "value-invalid"].contains(&code) || (code == "length-invalid" && (path.ends_with("/ease") || path.ends_with("/between")))
                });
                if !structural {
                    disagreements.push(format!("mutant {} {}: the twin refuses a document in which typescript finds no structural fault: {}", mutant["index"], mutant["applied"], mutant["issues"]));
                }
            }
        }
    }
    println!("[findings] mutants={} decoded={decoded} refused={refused} valid={valid} findings={findings} disagreements={}", entries(&document["mutants"]).len(), disagreements.len());
    for disagreement in disagreements.iter().take(12) {
        println!("[findings] {disagreement}");
    }
    assert!(decoded > 1000, "{decoded}");
    assert!(disagreements.is_empty());
}
