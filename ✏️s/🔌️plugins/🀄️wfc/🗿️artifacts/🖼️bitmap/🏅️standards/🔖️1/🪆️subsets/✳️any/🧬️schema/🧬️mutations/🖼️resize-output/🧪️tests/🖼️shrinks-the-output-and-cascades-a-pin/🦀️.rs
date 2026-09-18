//! 🧪️ `resize-output` fixture — `🖼️shrinks-the-output-and-cascades-a-pin`.
//!
//! the output lane plus the one pin the shrink strands, announced by an `info` cascade diagnostic.
//!
//! Source of truth is the committed JSON quintet beside this file, itself emitted by the artifact
//! root's own `emit_committed_fixtures` generator — a hand-edited fixture is a bug by construction.

use crate::diff::BitmapDiff;
use crate::mutations::{apply_bitmap_mutation, inverse_bitmap_mutation, BitmapMutation};
use crate::schema::snapshot::BitmapSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️resize-output/🖼️shrinks-the-output-and-cascades-a-pin/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️resize-output/🖼️shrinks-the-output-and-cascades-a-pin/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️resize-output/🖼️shrinks-the-output-and-cascades-a-pin/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️resize-output/🖼️shrinks-the-output-and-cascades-a-pin/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️resize-output/🖼️shrinks-the-output-and-cascades-a-pin/🎯️outcome/🔣️.json");

fn before() -> BitmapSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> BitmapSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> BitmapMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_bitmap_mutation(&mut snapshot, &mutation()).expect("resize-output applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "resize-output/🖼️shrinks-the-output-and-cascades-a-pin: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly — value AND position.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_bitmap_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_bitmap_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_bitmap_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "resize-output/🖼️shrinks-the-output-and-cascades-a-pin: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: BitmapSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "resize-output/🖼️shrinks-the-output-and-cascades-a-pin: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "resize-output/🖼️shrinks-the-output-and-cascades-a-pin: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <BitmapMutation as protocol::Mutation<BitmapSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "resize-output/🖼️shrinks-the-output-and-cascades-a-pin: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = apply_bitmap_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => {
            assert!(applied, "resize-output/🖼️shrinks-the-output-and-cascades-a-pin: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "resize-output/🖼️shrinks-the-output-and-cascades-a-pin: declared applied but the snapshot came back unchanged");
        }
        "rejected" => assert_eq!(snapshot, before(), "resize-output/🖼️shrinks-the-output-and-cascades-a-pin: a rejected mutation must leave the snapshot untouched"),
        other => panic!("resize-output/🖼️shrinks-the-output-and-cascades-a-pin: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the assertion that
/// pins WHICH lanes `resize-output` is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let raised = <BitmapMutation as protocol::Mutation<BitmapSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "resize-output/🖼️shrinks-the-output-and-cascades-a-pin: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: BitmapDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "resize-output/🖼️shrinks-the-output-and-cascades-a-pin: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `resize-output` changed, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: BitmapDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <BitmapDiff as protocol::MutationDiff<BitmapSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "resize-output/🖼️shrinks-the-output-and-cascades-a-pin: committed diff did not carry before to after");
}
