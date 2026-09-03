//! 🧪️ `connect-synapse` fixture — `joins-note-b-to-note-c-at-index-1`.
//!
//! `connect-synapse`'s diff builder clears four guards — duplicate synapse id, source widget present, target widget present, and no existing edge with the same from/from-port/to/to-port quadruple — before publishing the fixture with the new edge inserted.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::{apply_generation2d_mutation, inverse_generation2d_mutation, Generation2dMutation};
use crate::artifacts::generation2d::Generation2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Generation2dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Generation2dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Generation2dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_generation2d_mutation(&mut snapshot, &mutation()).expect("connect-synapse applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "connect-synapse/joins-note-b-to-note-c-at-index-1: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_generation2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_generation2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_generation2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "connect-synapse/joins-note-b-to-note-c-at-index-1: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Generation2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "connect-synapse/joins-note-b-to-note-c-at-index-1: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "connect-synapse/joins-note-b-to-note-c-at-index-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Generation2dMutation as protocol::Mutation<Generation2dSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "connect-synapse/joins-note-b-to-note-c-at-index-1: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = apply_generation2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => {
            assert!(applied, "connect-synapse/joins-note-b-to-note-c-at-index-1: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "connect-synapse/joins-note-b-to-note-c-at-index-1: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "connect-synapse/joins-note-b-to-note-c-at-index-1: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("connect-synapse/joins-note-b-to-note-c-at-index-1: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields `connect-synapse` is
/// allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let raised = <Generation2dMutation as protocol::Mutation<Generation2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "connect-synapse/joins-note-b-to-note-c-at-index-1: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: Generation2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "connect-synapse/joins-note-b-to-note-c-at-index-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `connect-synapse` changed, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: Generation2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Generation2dDiff as protocol::MutationDiff<Generation2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "connect-synapse/joins-note-b-to-note-c-at-index-1: committed diff did not carry before to after");
}
