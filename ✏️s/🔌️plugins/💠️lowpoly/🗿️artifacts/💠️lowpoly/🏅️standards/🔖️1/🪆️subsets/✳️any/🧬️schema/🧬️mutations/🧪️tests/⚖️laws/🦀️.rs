//! ⚖️ The laws every committed quintet of a single-row-inverse lowpoly leaf holds (`apply-paint-stroke`, the three selection motions and `set-vertex-positions`), written once and called by each scenario's `🧪️tests/<case>/🦀️.rs`.

use crate::{LowpolyDiff, LowpolyMutation, LowpolySnapshot};

fn from_json<T: semio_framework_value::FromValue>(text: &str) -> T {
    crate::standards::v1::subsets::any::io::text::lowpoly_json_decode(text).expect("fixture json decodes")
}

fn to_json<T: semio_framework_value::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&crate::standards::v1::subsets::any::io::text::lowpoly_json_encode(value)).expect("physical fixture JSON")
}

fn produced(mutation: &LowpolyMutation, before: &LowpolySnapshot) -> Vec<(String, String)> {
    <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::diff(mutation, before)
        .messages()
        .iter()
        .map(|message| (to_json(&message.level).as_str().unwrap_or_default().to_string(), message.code.0.clone()))
        .collect()
}

/// 📥️ The committed `before` snapshot and mutation, decoded.
pub fn decode_case(before: &str, mutation: &str) -> (LowpolySnapshot, LowpolyMutation) {
    (from_json(before), from_json(mutation))
}

/// ▶️ The leaf carries `before` to exactly the committed `after` and produces exactly the committed delta.
pub fn forward(before: &str, mutation: &str, after: &str, diff: &str) {
    let (before, mutation, after): (LowpolySnapshot, LowpolyMutation, LowpolySnapshot) = (from_json(before), from_json(mutation), from_json(after));
    let raised = <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::diff(&mutation, &before);
    assert_eq!(to_json(raised.diff()), serde_json::from_str::<serde_json::Value>(diff).expect("diff parses"), "the produced delta is the committed 🔺️diff");
    let (applied, _) = protocol::apply_mutation(&before, &mutation).expect("the leaf applies");
    assert_eq!(applied, after, "the applied layer is the committed after-snapshot");
    let committed: LowpolyDiff = from_json(diff);
    assert_eq!(protocol::apply_diff(&committed, &before).expect("the committed diff applies"), after, "the committed diff alone carries before to after");
}

/// ↩️ The computed inverse — ONE row writing the overwritten state back — restores `before` exactly.
pub fn inverse_restores(before: &str, mutation: &str) {
    let (base, mutation): (LowpolySnapshot, LowpolyMutation) = (from_json(before), from_json(mutation));
    let inverse = <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "the leaf inverts to ONE row: {inverse:?}");
    let (mut snapshot, _) = protocol::apply_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = protocol::apply_mutation(&snapshot, step).expect("inverse step applies").0;
    }
    assert_eq!(snapshot, base, "the inverse restores the before-snapshot");
}

/// 🎯️ The declared outcome — status and ordered diagnostics — is what the leaf emits.
pub fn declared_outcome(before: &str, mutation: &str, outcome: &str) {
    let (before, mutation): (LowpolySnapshot, LowpolyMutation) = (from_json(before), from_json(mutation));
    let outcome: serde_json::Value = serde_json::from_str(outcome).expect("outcome parses");
    let declared: Vec<(String, String)> = match outcome["status"].as_str() {
        Some("rejected") => { let code = outcome["code"].as_str().unwrap_or_default(); vec![(to_json(&protocol::outcome_code_level(code).expect("a vocabulary code")).as_str().unwrap_or_default().to_string(), code.to_string())] }
        _ => outcome["messages"].as_array().map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default(),
    };
    assert_eq!(produced(&mutation, &before), declared, "the raised diagnostics are the committed 🎯️outcome");
}

/// ⛔️ A refused or no-op leaf leaves the document byte-identical and emits the declared diagnostic.
pub fn refusal(before: &str, mutation: &str, after: &str, outcome: &str) {
    declared_outcome(before, mutation, outcome);
    let (base, mutation, after): (LowpolySnapshot, LowpolyMutation, LowpolySnapshot) = (from_json(before), from_json(mutation), from_json(after));
    assert_eq!(after, base, "the committed after-snapshot is the before-snapshot");
    let snapshot = protocol::apply_mutation(&base, &mutation).map_or_else(|_| base.clone(), |(next, _)| next);
    assert_eq!(snapshot, base, "a refused or no-op leaf leaves the document untouched");
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
pub fn canonical(before: &str, after: &str, mutation: &str, diff: Option<&str>) {
    for text in [before, after] {
        assert_eq!(to_json(&from_json::<LowpolySnapshot>(text)), serde_json::from_str::<serde_json::Value>(text).expect("snapshot parses"), "a committed snapshot is canonical");
    }
    assert_eq!(to_json(&from_json::<LowpolyMutation>(mutation)), serde_json::from_str::<serde_json::Value>(mutation).expect("mutation parses"), "the committed mutation is canonical");
    if let Some(diff) = diff {
        assert_eq!(to_json(&from_json::<LowpolyDiff>(diff)), serde_json::from_str::<serde_json::Value>(diff).expect("diff parses"), "the committed diff is canonical");
    }
}
