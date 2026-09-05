//! 🧪️ `delete-widget-position` fixture — `🧹️unpins-the-node-a-position`.
//!
//! `delete-widget-position`'s diff builder guards on BOTH the widget existing and it actually having a layout entry, then removes that key — the widget itself stays in the fixture, merely unpositioned.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::mutations::{apply_generation3d_mutation, inverse_generation3d_mutation, Generation3dMutation};
use crate::artifacts::generation3d::Generation3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Generation3dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Generation3dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Generation3dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_generation3d_mutation(&mut snapshot, &mutation()).expect("delete-widget-position applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-widget-position/unpins-the-node-a-position: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_generation3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_generation3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_generation3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-widget-position/unpins-the-node-a-position: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Generation3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-widget-position/unpins-the-node-a-position: committed {side} JSON is not canonical");
    }
    let reencoded: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-widget-position/unpins-the-node-a-position: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Generation3dMutation as protocol::Mutation<Generation3dSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            (format!("{:?}", message.level).to_lowercase(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "delete-widget-position/unpins-the-node-a-position: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = apply_generation3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => {
            assert!(applied, "delete-widget-position/unpins-the-node-a-position: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "delete-widget-position/unpins-the-node-a-position: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "delete-widget-position/unpins-the-node-a-position: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("delete-widget-position/unpins-the-node-a-position: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields `delete-widget-position` is
/// allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let raised = <Generation3dMutation as protocol::Mutation<Generation3dSnapshot>>::diff(&mutation(), &base);
    let produced: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-widget-position/unpins-the-node-a-position: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: Generation3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-widget-position/unpins-the-node-a-position: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `delete-widget-position` changed, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: Generation3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <Generation3dDiff as protocol::MutationDiff<Generation3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-widget-position/unpins-the-node-a-position: committed diff did not carry before to after");
}
