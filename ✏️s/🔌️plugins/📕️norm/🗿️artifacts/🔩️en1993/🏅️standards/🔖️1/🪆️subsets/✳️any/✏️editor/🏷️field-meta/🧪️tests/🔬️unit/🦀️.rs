//! 🧪️ `tests` — moved out of `🏷️field-meta/🦀️.rs` into its canonical test implementation.
use super::*;
use crate::En1993Snapshot;

fn to_template(path: &str) -> String {
    let mut out = String::new();
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' {
            out.push_str("[]");
            while let Some(x) = chars.next() {
                if x == ']' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn walk(path: &str, value: &serde_json::Value, missing: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let child = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                if matches!(v, serde_json::Value::Object(_) | serde_json::Value::Array(_)) {
                    if en1993_field_meta(&child).or_else(|| en1993_field_meta(&to_template(&child))).is_none() {
                        // containers still need labels
                        missing.push(child.clone());
                    }
                    walk(&child, v, missing);
                } else {
                    let meta = en1993_field_meta(&child).or_else(|| en1993_field_meta(&to_template(&child)));
                    if meta.is_none() {
                        missing.push(child);
                    } else if let Some(meta) = meta {
                        assert!(!meta.label_en.is_empty() && !meta.label_de.is_empty(), "{child}");
                        let _ = meta.unit; // SI or explicit none
                    }
                }
            }
        }
        serde_json::Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                walk(&format!("{path}[{i}]"), v, missing);
            }
        }
        _ => {}
    }
}

#[test]
fn every_default_snapshot_editable_leaf_has_en_de_meta() {
    let doc = En1993Snapshot::default();
    let json = serde_json::to_value(&doc).expect("serialize");
    let mut missing = Vec::new();
    walk("", &json, &mut missing);
    assert!(missing.is_empty(), "missing field-meta for: {missing:?}");
}

#[test]
fn annex_choices_are_human_labels() {
    let meta = en1993_field_meta("annex").expect("annex");
    let choices = meta.choices.expect("choices");
    assert_eq!(choices[0].label_en, "EN (CEN)");
    assert_eq!(choices[1].label_de, "Deutschland (DIN)");
}
