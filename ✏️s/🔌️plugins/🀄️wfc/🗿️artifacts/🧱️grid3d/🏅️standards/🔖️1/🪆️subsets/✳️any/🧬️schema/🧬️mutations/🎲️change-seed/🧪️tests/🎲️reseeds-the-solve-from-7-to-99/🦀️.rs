//! 🧪️ `change-seed` fixture — `🎲️reseeds-the-solve-from-7-to-99`.
//!
//! `change-seed` writes ONE scalar lane; every collection lane stays empty.
//!
//! Source of truth is the committed JSON quintet beside this file; the Rust builder that produced it
//! is `🧪️tests/🔬️unit`'s own `fixture_base`/`fixture_vectors`, so a fixture can never disagree with
//! the mutation it claims to encode.

use crate::diff::Grid3dDiff;
use crate::mutations::{apply_grid3d_mutation, inverse_grid3d_mutation, Grid3dMutation};
use crate::schema::snapshot::Grid3dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/🎯️outcome/🔣️.json");

fn before() -> Grid3dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Grid3dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Grid3dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_grid3d_mutation(&mut snapshot, &mutation()).expect("change-seed applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-seed/🎲️reseeds-the-solve-from-7-to-99: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly — row POSITION included.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_grid3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_grid3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_grid3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-seed/🎲️reseeds-the-solve-from-7-to-99: the inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Grid3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-seed/🎲️reseeds-the-solve-from-7-to-99: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-seed/🎲️reseeds-the-solve-from-7-to-99: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <Grid3dMutation as protocol::Mutation<Grid3dSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "change-seed/🎲️reseeds-the-solve-from-7-to-99: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = apply_grid3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => {
            assert!(applied, "change-seed/🎲️reseeds-the-solve-from-7-to-99: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "change-seed/🎲️reseeds-the-solve-from-7-to-99: declared applied but the snapshot came back unchanged");
        }
        "rejected" => assert_eq!(snapshot, before(), "change-seed/🎲️reseeds-the-solve-from-7-to-99: a rejected mutation must leave the snapshot untouched"),
        other => panic!("change-seed/🎲️reseeds-the-solve-from-7-to-99: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the assertion that pins
/// WHICH lanes this kind is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let raised = <Grid3dMutation as protocol::Mutation<Grid3dSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-seed/🎲️reseeds-the-solve-from-7-to-99: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: Grid3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-seed/🎲️reseeds-the-solve-from-7-to-99: committed diff JSON is not canonical");
}

/// 🩹️ Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// COMPLETE description of what this kind changed, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: Grid3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <Grid3dDiff as protocol::MutationDiff<Grid3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-seed/🎲️reseeds-the-solve-from-7-to-99: committed diff did not carry before to after");
}
