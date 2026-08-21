//! 🧪️ `rename-human-factor-requirement` fixture — `renames-human-factor-requirement-a`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️component.rs`, which patches only `name` on `human-factor-requirement-a` in `program.human_factors` — every other patch field stays `null`.
//!
//! That leaf's own contract line reads: ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
//!
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from this JSON by `fixtures generate` and are asserted by the shared codec-matrix harness.

use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> ProgramSnapshot {
    serde_json::from_str(BEFORE).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: mutation decodes")
}

/// ▶️ rename-human-factor-requirement carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn rename_human_factor_requirement_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: rename-human-factor-requirement applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "rename-human-factor-requirement/renames-human-factor-requirement-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying rename-human-factor-requirement and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn rename_human_factor_requirement_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: inverse step applies");
    }
    assert_eq!(state, base, "rename-human-factor-requirement/renames-human-factor-requirement-a: rename-human-factor-requirement (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed rename-human-factor-requirement payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn rename_human_factor_requirement_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: snapshot reparses");
        assert_eq!(reencoded, original, "rename-human-factor-requirement/renames-human-factor-requirement-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: rename-human-factor-requirement payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: rename-human-factor-requirement payload reparses");
    assert_eq!(reencoded, original, "rename-human-factor-requirement/renames-human-factor-requirement-a: committed rename-human-factor-requirement payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: rename-human-factor-requirement applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn rename_human_factor_requirement_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-human-factor-requirement/renames-human-factor-requirement-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "rename-human-factor-requirement/renames-human-factor-requirement-a: rename-human-factor-requirement raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "rename-human-factor-requirement/renames-human-factor-requirement-a: rename-human-factor-requirement was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta rename-human-factor-requirement produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn rename_human_factor_requirement_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: committed diff decodes");
    assert_eq!(produced, committed, "rename-human-factor-requirement/renames-human-factor-requirement-a: the diff rename-human-factor-requirement builds differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn rename_human_factor_requirement_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: committed diff reparses");
    assert_eq!(reencoded, original, "rename-human-factor-requirement/renames-human-factor-requirement-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what rename-human-factor-requirement does, not a summary.
#[semio_framework_async_macros::async_test]
async fn rename_human_factor_requirement_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: committed diff decodes");
    let produced = decoded.apply(&before()).expect("rename-human-factor-requirement/renames-human-factor-requirement-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-human-factor-requirement/renames-human-factor-requirement-a: the committed diff did not carry before to after");
}
