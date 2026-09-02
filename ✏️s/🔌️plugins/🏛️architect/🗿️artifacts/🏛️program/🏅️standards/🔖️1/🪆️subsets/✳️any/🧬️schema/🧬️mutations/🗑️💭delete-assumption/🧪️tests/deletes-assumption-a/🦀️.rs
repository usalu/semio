//! 🧪️ `delete-assumption` fixture — `deletes-assumption-a`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which drops `assumption-a` from `program.assumptions` as `removed = ["assumption-a"]`, after a target-missing guard.
//!
//! That leaf's own contract line reads: 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
//!
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from this JSON by `fixtures generate` and are asserted by the shared codec-matrix harness.

use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> ProgramSnapshot {
    serde_json::from_str(BEFORE).expect("delete-assumption/deletes-assumption-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("delete-assumption/deletes-assumption-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("delete-assumption/deletes-assumption-a: mutation decodes")
}

/// ▶️ delete-assumption carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn delete_assumption_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("delete-assumption/deletes-assumption-a: delete-assumption applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-assumption/deletes-assumption-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying delete-assumption and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn delete_assumption_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("delete-assumption/deletes-assumption-a: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("delete-assumption/deletes-assumption-a: inverse step applies");
    }
    assert_eq!(state, base, "delete-assumption/deletes-assumption-a: create-assumption (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed delete-assumption payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn delete_assumption_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("delete-assumption/deletes-assumption-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("delete-assumption/deletes-assumption-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("delete-assumption/deletes-assumption-a: snapshot reparses");
        assert_eq!(reencoded, original, "delete-assumption/deletes-assumption-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-assumption/deletes-assumption-a: delete-assumption payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-assumption/deletes-assumption-a: delete-assumption payload reparses");
    assert_eq!(reencoded, original, "delete-assumption/deletes-assumption-a: committed delete-assumption payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: delete-assumption applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn delete_assumption_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("delete-assumption/deletes-assumption-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-assumption/deletes-assumption-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "delete-assumption/deletes-assumption-a: delete-assumption raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "delete-assumption/deletes-assumption-a: delete-assumption was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta delete-assumption produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn delete_assumption_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("delete-assumption/deletes-assumption-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("delete-assumption/deletes-assumption-a: committed diff decodes");
    assert_eq!(produced, committed, "delete-assumption/deletes-assumption-a: the diff delete-assumption builds differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn delete_assumption_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("delete-assumption/deletes-assumption-a: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("delete-assumption/deletes-assumption-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("delete-assumption/deletes-assumption-a: committed diff reparses");
    assert_eq!(reencoded, original, "delete-assumption/deletes-assumption-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what delete-assumption does, not a summary.
#[semio_framework_async_macros::async_test]
async fn delete_assumption_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("delete-assumption/deletes-assumption-a: committed diff decodes");
    let produced = decoded.apply(&before()).expect("delete-assumption/deletes-assumption-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-assumption/deletes-assumption-a: the committed diff did not carry before to after");
}
