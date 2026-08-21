//! 🧪️ `rename-assumption` fixture — `renames-assumption-a`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️component.rs`, which patches only `name` on `assumption-a` in `program.assumptions` — every other patch field stays `null`.
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
    serde_json::from_str(BEFORE).expect("rename-assumption/renames-assumption-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("rename-assumption/renames-assumption-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("rename-assumption/renames-assumption-a: mutation decodes")
}

/// ▶️ rename-assumption carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn rename_assumption_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("rename-assumption/renames-assumption-a: rename-assumption applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "rename-assumption/renames-assumption-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying rename-assumption and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn rename_assumption_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("rename-assumption/renames-assumption-a: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("rename-assumption/renames-assumption-a: inverse step applies");
    }
    assert_eq!(state, base, "rename-assumption/renames-assumption-a: rename-assumption (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed rename-assumption payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn rename_assumption_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("rename-assumption/renames-assumption-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("rename-assumption/renames-assumption-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("rename-assumption/renames-assumption-a: snapshot reparses");
        assert_eq!(reencoded, original, "rename-assumption/renames-assumption-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("rename-assumption/renames-assumption-a: rename-assumption payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("rename-assumption/renames-assumption-a: rename-assumption payload reparses");
    assert_eq!(reencoded, original, "rename-assumption/renames-assumption-a: committed rename-assumption payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: rename-assumption applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn rename_assumption_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("rename-assumption/renames-assumption-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-assumption/renames-assumption-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "rename-assumption/renames-assumption-a: rename-assumption raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "rename-assumption/renames-assumption-a: rename-assumption was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta rename-assumption produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn rename_assumption_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("rename-assumption/renames-assumption-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("rename-assumption/renames-assumption-a: committed diff decodes");
    assert_eq!(produced, committed, "rename-assumption/renames-assumption-a: the diff rename-assumption builds differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn rename_assumption_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("rename-assumption/renames-assumption-a: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("rename-assumption/renames-assumption-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("rename-assumption/renames-assumption-a: committed diff reparses");
    assert_eq!(reencoded, original, "rename-assumption/renames-assumption-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what rename-assumption does, not a summary.
#[semio_framework_async_macros::async_test]
async fn rename_assumption_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("rename-assumption/renames-assumption-a: committed diff decodes");
    let produced = decoded.apply(&before()).expect("rename-assumption/renames-assumption-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-assumption/renames-assumption-a: the committed diff did not carry before to after");
}
