//! 🧪️ `delete-growth-plan` fixture — `deletes-growth-plan-a`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which drops `growth-plan-a` from `program.growth` as `removed = ["growth-plan-a"]`, after a target-missing guard.
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
    serde_json::from_str(BEFORE).expect("delete-growth-plan/deletes-growth-plan-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("delete-growth-plan/deletes-growth-plan-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("delete-growth-plan/deletes-growth-plan-a: mutation decodes")
}

/// ▶️ delete-growth-plan carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn delete_growth_plan_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("delete-growth-plan/deletes-growth-plan-a: delete-growth-plan applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-growth-plan/deletes-growth-plan-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying delete-growth-plan and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn delete_growth_plan_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("delete-growth-plan/deletes-growth-plan-a: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("delete-growth-plan/deletes-growth-plan-a: inverse step applies");
    }
    assert_eq!(state, base, "delete-growth-plan/deletes-growth-plan-a: create-growth-plan (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed delete-growth-plan payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn delete_growth_plan_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("delete-growth-plan/deletes-growth-plan-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("delete-growth-plan/deletes-growth-plan-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("delete-growth-plan/deletes-growth-plan-a: snapshot reparses");
        assert_eq!(reencoded, original, "delete-growth-plan/deletes-growth-plan-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-growth-plan/deletes-growth-plan-a: delete-growth-plan payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-growth-plan/deletes-growth-plan-a: delete-growth-plan payload reparses");
    assert_eq!(reencoded, original, "delete-growth-plan/deletes-growth-plan-a: committed delete-growth-plan payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: delete-growth-plan applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn delete_growth_plan_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("delete-growth-plan/deletes-growth-plan-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-growth-plan/deletes-growth-plan-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "delete-growth-plan/deletes-growth-plan-a: delete-growth-plan raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "delete-growth-plan/deletes-growth-plan-a: delete-growth-plan was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta delete-growth-plan produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn delete_growth_plan_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("delete-growth-plan/deletes-growth-plan-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("delete-growth-plan/deletes-growth-plan-a: committed diff decodes");
    assert_eq!(produced, committed, "delete-growth-plan/deletes-growth-plan-a: the diff delete-growth-plan builds differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn delete_growth_plan_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("delete-growth-plan/deletes-growth-plan-a: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("delete-growth-plan/deletes-growth-plan-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("delete-growth-plan/deletes-growth-plan-a: committed diff reparses");
    assert_eq!(reencoded, original, "delete-growth-plan/deletes-growth-plan-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what delete-growth-plan does, not a summary.
#[semio_framework_async_macros::async_test]
async fn delete_growth_plan_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("delete-growth-plan/deletes-growth-plan-a: committed diff decodes");
    let produced = decoded.apply(&before()).expect("delete-growth-plan/deletes-growth-plan-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-growth-plan/deletes-growth-plan-a: the committed diff did not carry before to after");
}
