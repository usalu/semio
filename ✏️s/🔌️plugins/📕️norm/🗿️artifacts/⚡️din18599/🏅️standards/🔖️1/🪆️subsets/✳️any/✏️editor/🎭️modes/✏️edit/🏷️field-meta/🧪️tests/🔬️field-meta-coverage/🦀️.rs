//! 🧪️ `field_meta_coverage` — moved out of `🏷️field-meta/🦀️.rs` into its canonical test implementation.
use super::*;
use crate::Din18599Snapshot;

#[test]
fn every_default_leaf_has_en_de_label() {
    let doc = Din18599Snapshot::default();
    let json = crate::standards::v1::subsets::any::schema::snapshot::encode_din18599_snapshot_json(&doc);
    let value: serde_json::Value = serde_json::from_str(&json).expect("json");
    let mut missing = Vec::new();
    walk("", &value, &mut missing);
    assert!(missing.is_empty(), "missing field-meta for: {missing:?}");
}

fn walk(path: &str, value: &serde_json::Value, missing: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let child = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                if matches!(v, serde_json::Value::Object(_) | serde_json::Value::Array(_)) {
                    walk(&child, v, missing);
                } else if field_meta(&child).is_none() && field_meta(&to_template(&child)).is_none() {
                    missing.push(child);
                } else if let Some(meta) = field_meta(&child).or_else(|| field_meta(&to_template(&child))) {
                    assert!(!meta.label_en.is_empty() && !meta.label_de.is_empty(), "{child}");
                }
            }
        }
        serde_json::Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                let child = format!("{path}[{i}]");
                walk(&child, v, missing);
            }
        }
        _ => {}
    }
}

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
