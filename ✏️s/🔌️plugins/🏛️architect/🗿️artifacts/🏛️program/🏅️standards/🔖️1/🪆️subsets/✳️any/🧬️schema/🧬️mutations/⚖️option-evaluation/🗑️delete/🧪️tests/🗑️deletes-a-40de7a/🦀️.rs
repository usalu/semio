//! 🧪️ `delete-option-evaluation` fixture — `🗑️deletes-a`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which drops `option-evaluation-a` from `program.options` as `removed = ["option-evaluation-a"]`, after a target-missing guard.
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
    serde_json::from_str(BEFORE).expect("delete-option-evaluation/deletes-option-evaluation-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("delete-option-evaluation/deletes-option-evaluation-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("delete-option-evaluation/deletes-option-evaluation-a: mutation decodes")
}

/// ▶️ delete-option-evaluation carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn delete_option_evaluation_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("delete-option-evaluation/deletes-option-evaluation-a: delete-option-evaluation applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-option-evaluation/deletes-option-evaluation-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying delete-option-evaluation and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn delete_option_evaluation_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("delete-option-evaluation/deletes-option-evaluation-a: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("delete-option-evaluation/deletes-option-evaluation-a: inverse step applies");
    }
    assert_eq!(state, base, "delete-option-evaluation/deletes-option-evaluation-a: create-option-evaluation (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed delete-option-evaluation payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn delete_option_evaluation_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("delete-option-evaluation/deletes-option-evaluation-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("delete-option-evaluation/deletes-option-evaluation-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("delete-option-evaluation/deletes-option-evaluation-a: snapshot reparses");
        assert_eq!(reencoded, original, "delete-option-evaluation/deletes-option-evaluation-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-option-evaluation/deletes-option-evaluation-a: delete-option-evaluation payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-option-evaluation/deletes-option-evaluation-a: delete-option-evaluation payload reparses");
    assert_eq!(reencoded, original, "delete-option-evaluation/deletes-option-evaluation-a: committed delete-option-evaluation payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: delete-option-evaluation applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn delete_option_evaluation_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("delete-option-evaluation/deletes-option-evaluation-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-option-evaluation/deletes-option-evaluation-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "delete-option-evaluation/deletes-option-evaluation-a: delete-option-evaluation raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "delete-option-evaluation/deletes-option-evaluation-a: delete-option-evaluation was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta delete-option-evaluation produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn delete_option_evaluation_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("delete-option-evaluation/deletes-option-evaluation-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("delete-option-evaluation/deletes-option-evaluation-a: committed diff decodes");
    assert_eq!(produced, committed, "delete-option-evaluation/deletes-option-evaluation-a: the diff delete-option-evaluation builds differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn delete_option_evaluation_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("delete-option-evaluation/deletes-option-evaluation-a: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("delete-option-evaluation/deletes-option-evaluation-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("delete-option-evaluation/deletes-option-evaluation-a: committed diff reparses");
    assert_eq!(reencoded, original, "delete-option-evaluation/deletes-option-evaluation-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what delete-option-evaluation does, not a summary.
#[semio_framework_async_macros::async_test]
async fn delete_option_evaluation_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("delete-option-evaluation/deletes-option-evaluation-a: committed diff decodes");
    let produced = decoded.apply(&before()).expect("delete-option-evaluation/deletes-option-evaluation-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-option-evaluation/deletes-option-evaluation-a: the committed diff did not carry before to after");
}
