//! 🧪️ `delete-survey` fixture — `🗑️deletes-a`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which drops `survey-a` from `program.surveys` as `removed = ["survey-a"]`, after a target-missing guard.
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
    serde_json::from_str(BEFORE).expect("delete-survey/deletes-survey-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("delete-survey/deletes-survey-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("delete-survey/deletes-survey-a: mutation decodes")
}

/// ▶️ delete-survey carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn delete_survey_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("delete-survey/deletes-survey-a: delete-survey applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-survey/deletes-survey-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying delete-survey and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn delete_survey_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("delete-survey/deletes-survey-a: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("delete-survey/deletes-survey-a: inverse step applies");
    }
    assert_eq!(state, base, "delete-survey/deletes-survey-a: create-survey (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed delete-survey payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn delete_survey_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("delete-survey/deletes-survey-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("delete-survey/deletes-survey-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("delete-survey/deletes-survey-a: snapshot reparses");
        assert_eq!(reencoded, original, "delete-survey/deletes-survey-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-survey/deletes-survey-a: delete-survey payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-survey/deletes-survey-a: delete-survey payload reparses");
    assert_eq!(reencoded, original, "delete-survey/deletes-survey-a: committed delete-survey payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: delete-survey applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn delete_survey_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("delete-survey/deletes-survey-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-survey/deletes-survey-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "delete-survey/deletes-survey-a: delete-survey raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "delete-survey/deletes-survey-a: delete-survey was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta delete-survey produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn delete_survey_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("delete-survey/deletes-survey-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("delete-survey/deletes-survey-a: committed diff decodes");
    assert_eq!(produced, committed, "delete-survey/deletes-survey-a: the diff delete-survey builds differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn delete_survey_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("delete-survey/deletes-survey-a: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("delete-survey/deletes-survey-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("delete-survey/deletes-survey-a: committed diff reparses");
    assert_eq!(reencoded, original, "delete-survey/deletes-survey-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what delete-survey does, not a summary.
#[semio_framework_async_macros::async_test]
async fn delete_survey_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("delete-survey/deletes-survey-a: committed diff decodes");
    let produced = decoded.apply(&before()).expect("delete-survey/deletes-survey-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-survey/deletes-survey-a: the committed diff did not carry before to after");
}
