//! 🧪️ `replace-constraint-record` fixture — `replaces-constraint-record-a`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which patches `constraint-record-a` in `program.constraints` with the FULL `None` that `Patchable::diff_patch` snapshots off the payload row (this fixture moves `constraint_type`).
//!
//! That leaf's own contract line reads: 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
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
    serde_json::from_str(BEFORE).expect("replace-constraint-record/replaces-constraint-record-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("replace-constraint-record/replaces-constraint-record-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("replace-constraint-record/replaces-constraint-record-a: mutation decodes")
}

/// ▶️ replace-constraint-record carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn replace_constraint_record_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("replace-constraint-record/replaces-constraint-record-a: replace-constraint-record applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-constraint-record/replaces-constraint-record-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying replace-constraint-record and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn replace_constraint_record_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("replace-constraint-record/replaces-constraint-record-a: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("replace-constraint-record/replaces-constraint-record-a: inverse step applies");
    }
    assert_eq!(state, base, "replace-constraint-record/replaces-constraint-record-a: replace-constraint-record (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed replace-constraint-record payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn replace_constraint_record_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("replace-constraint-record/replaces-constraint-record-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("replace-constraint-record/replaces-constraint-record-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("replace-constraint-record/replaces-constraint-record-a: snapshot reparses");
        assert_eq!(reencoded, original, "replace-constraint-record/replaces-constraint-record-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("replace-constraint-record/replaces-constraint-record-a: replace-constraint-record payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("replace-constraint-record/replaces-constraint-record-a: replace-constraint-record payload reparses");
    assert_eq!(reencoded, original, "replace-constraint-record/replaces-constraint-record-a: committed replace-constraint-record payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: replace-constraint-record applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn replace_constraint_record_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("replace-constraint-record/replaces-constraint-record-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-constraint-record/replaces-constraint-record-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "replace-constraint-record/replaces-constraint-record-a: replace-constraint-record raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "replace-constraint-record/replaces-constraint-record-a: replace-constraint-record was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta replace-constraint-record produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn replace_constraint_record_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("replace-constraint-record/replaces-constraint-record-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("replace-constraint-record/replaces-constraint-record-a: committed diff decodes");
    assert_eq!(produced, committed, "replace-constraint-record/replaces-constraint-record-a: the diff replace-constraint-record builds differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn replace_constraint_record_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("replace-constraint-record/replaces-constraint-record-a: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("replace-constraint-record/replaces-constraint-record-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("replace-constraint-record/replaces-constraint-record-a: committed diff reparses");
    assert_eq!(reencoded, original, "replace-constraint-record/replaces-constraint-record-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what replace-constraint-record does, not a summary.
#[semio_framework_async_macros::async_test]
async fn replace_constraint_record_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("replace-constraint-record/replaces-constraint-record-a: committed diff decodes");
    let produced = decoded.apply(&before()).expect("replace-constraint-record/replaces-constraint-record-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-constraint-record/replaces-constraint-record-a: the committed diff did not carry before to after");
}
