//! 🔬️ Ticket probe of work package L: counts, per pets fixture, the decimal literals that `serde_json` (as the scratch
//! crate links it) reads as another double than the correctly rounding `str::parse::<f64>` — the reader `JSON.parse`
//! and the test host's own JSON reader agree with. Without the feature `float_roundtrip` it is one literal in fifty;
//! with it, as the real crate links the codec, none.
//!
//! Mounted as the integration test `json_number_parsing` of the scratch crate (`[[test]] path = "../../../../../probe_json_number_parsing.rs"`).
//! Run from the repository root: `bash <ticket>/rust_scratch.sh wp-l test --offline --test json_number_parsing -- --nocapture`
//!
//! @see ./📓️report-wp-l.md — the recorded result and what follows from it

use std::path::PathBuf;

/// 🔢️ Every number literal of a JSON text, outside of strings.
fn literals(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        match bytes[at] {
            b'"' => {
                at += 1;
                while at < bytes.len() && bytes[at] != b'"' {
                    at += if bytes[at] == b'\\' { 2 } else { 1 };
                }
                at += 1;
            }
            b'-' | b'0'..=b'9' => {
                let start = at;
                while at < bytes.len() && matches!(bytes[at], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    at += 1;
                }
                found.push(&text[start..at]);
            }
            _ => at += 1,
        }
    }
    found
}

#[test]
fn count_the_literals_serde_json_reads_as_a_neighbouring_double() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🧫️fixtures");
    let mut cases: Vec<PathBuf> = std::fs::read_dir(&root).unwrap_or_else(|error| panic!("{}: {error}", root.display())).flatten().map(|entry| entry.path()).collect();
    cases.sort();
    let mut report = String::new();
    for case in cases {
        let text = std::fs::read_to_string(case.join("🔣️.json")).unwrap_or_else(|error| panic!("{}: {error}", case.display()));
        let literals = literals(&text);
        let apart: Vec<&str> = literals.iter().copied().filter(|literal| serde_json::from_str::<f64>(literal).ok().map(f64::to_bits) != literal.parse::<f64>().ok().map(f64::to_bits)).collect();
        let name = case.file_name().and_then(|name| name.to_str()).unwrap_or_default();
        let first = apart.first().map_or(String::new(), |literal| format!(" (first: {literal})"));
        report.push_str(&format!("{name}: {} number literals, {} read as another double by serde_json{first}\n", literals.len(), apart.len()));
    }
    std::fs::write(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../json-number-parsing.txt"), &report).unwrap_or_else(|error| panic!("{error}"));
    println!("{report}");
}
