//! 🧪️ `rename-project` fixture — `renames-the-project-code`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which clones `base.project`, writes the single `code` field, and emits it as the whole `project` facet — the diff carries the complete replacement value, not a patch.
//!
//! That leaf's own contract line reads: ✏️ New `ProjectDefinition` with only `code` changed. Root-scoped singleton — always present, so Warning `mutation.no-op` (empty diff) covers the only degenerate case: the code is unchanged.
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
    serde_json::from_str(BEFORE).expect("rename-project/renames-the-project-code: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("rename-project/renames-the-project-code: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("rename-project/renames-the-project-code: mutation decodes")
}

/// ▶️ rename-project carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn rename_project_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("rename-project/renames-the-project-code: rename-project applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "rename-project/renames-the-project-code: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying rename-project and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn rename_project_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("rename-project/renames-the-project-code: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("rename-project/renames-the-project-code: inverse step applies");
    }
    assert_eq!(state, base, "rename-project/renames-the-project-code: rename-project back to the captured prior value did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed rename-project payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn rename_project_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("rename-project/renames-the-project-code: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("rename-project/renames-the-project-code: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("rename-project/renames-the-project-code: snapshot reparses");
        assert_eq!(reencoded, original, "rename-project/renames-the-project-code: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("rename-project/renames-the-project-code: rename-project payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("rename-project/renames-the-project-code: rename-project payload reparses");
    assert_eq!(reencoded, original, "rename-project/renames-the-project-code: committed rename-project payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: rename-project applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn rename_project_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("rename-project/renames-the-project-code: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-project/renames-the-project-code: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "rename-project/renames-the-project-code: rename-project raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "rename-project/renames-the-project-code: rename-project was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta rename-project produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn rename_project_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("rename-project/renames-the-project-code: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("rename-project/renames-the-project-code: committed diff decodes");
    assert_eq!(produced, committed, "rename-project/renames-the-project-code: the diff rename-project builds differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn rename_project_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("rename-project/renames-the-project-code: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("rename-project/renames-the-project-code: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("rename-project/renames-the-project-code: committed diff reparses");
    assert_eq!(reencoded, original, "rename-project/renames-the-project-code: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what rename-project does, not a summary.
#[semio_framework_async_macros::async_test]
async fn rename_project_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("rename-project/renames-the-project-code: committed diff decodes");
    let produced = decoded.apply(&before()).expect("rename-project/renames-the-project-code: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-project/renames-the-project-code: the committed diff did not carry before to after");
}
