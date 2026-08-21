//! 🧪️ `connect-slots` fixture — `joins-slot-b-to-slot-c-at-index-1`.
//!
//! `connect-slots`'s diff builder writes ONE `edges_upserted` entry after clearing four guards: duplicate edge id, both endpoints present, no self-loop, and no parallel edge in either direction.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::mutations::{apply_assembly_mutation, inverse_assembly_mutation, AssemblyMutation};
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> AssemblySnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> AssemblySnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> AssemblyMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_assembly_mutation(&mut snapshot, &mutation()).expect("connect-slots applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "connect-slots/joins-slot-b-to-slot-c-at-index-1: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_assembly_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_assembly_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_assembly_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "connect-slots/joins-slot-b-to-slot-c-at-index-1: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: AssemblySnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "connect-slots/joins-slot-b-to-slot-c-at-index-1: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "connect-slots/joins-slot-b-to-slot-c-at-index-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <AssemblyMutation as protocol::Mutation<AssemblySnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "connect-slots/joins-slot-b-to-slot-c-at-index-1: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = apply_assembly_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => {
            assert!(applied, "connect-slots/joins-slot-b-to-slot-c-at-index-1: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "connect-slots/joins-slot-b-to-slot-c-at-index-1: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "connect-slots/joins-slot-b-to-slot-c-at-index-1: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("connect-slots/joins-slot-b-to-slot-c-at-index-1: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields `connect-slots` is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <AssemblyMutation as protocol::Mutation<AssemblySnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "connect-slots/joins-slot-b-to-slot-c-at-index-1: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: AssemblyDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "connect-slots/joins-slot-b-to-slot-c-at-index-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `connect-slots` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: AssemblyDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <AssemblyDiff as protocol::MutationDiff<AssemblySnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "connect-slots/joins-slot-b-to-slot-c-at-index-1: committed diff did not carry before to after");
}
