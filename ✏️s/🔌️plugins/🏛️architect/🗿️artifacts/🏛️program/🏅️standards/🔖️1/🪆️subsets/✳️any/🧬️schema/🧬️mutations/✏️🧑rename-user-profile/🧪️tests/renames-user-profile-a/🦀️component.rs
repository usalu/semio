//! 🧪️ `rename-user-profile` fixture — `renames-user-profile-a`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️component.rs`, which patches only `name` on `user-profile-a` in `program.users` — every other patch field stays `null`.
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
    serde_json::from_str(BEFORE).expect("rename-user-profile/renames-user-profile-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("rename-user-profile/renames-user-profile-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("rename-user-profile/renames-user-profile-a: mutation decodes")
}

/// ▶️ rename-user-profile carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn rename_user_profile_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("rename-user-profile/renames-user-profile-a: rename-user-profile applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "rename-user-profile/renames-user-profile-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying rename-user-profile and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn rename_user_profile_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("rename-user-profile/renames-user-profile-a: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("rename-user-profile/renames-user-profile-a: inverse step applies");
    }
    assert_eq!(state, base, "rename-user-profile/renames-user-profile-a: rename-user-profile (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed rename-user-profile payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn rename_user_profile_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("rename-user-profile/renames-user-profile-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("rename-user-profile/renames-user-profile-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("rename-user-profile/renames-user-profile-a: snapshot reparses");
        assert_eq!(reencoded, original, "rename-user-profile/renames-user-profile-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("rename-user-profile/renames-user-profile-a: rename-user-profile payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("rename-user-profile/renames-user-profile-a: rename-user-profile payload reparses");
    assert_eq!(reencoded, original, "rename-user-profile/renames-user-profile-a: committed rename-user-profile payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: rename-user-profile applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn rename_user_profile_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("rename-user-profile/renames-user-profile-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-user-profile/renames-user-profile-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "rename-user-profile/renames-user-profile-a: rename-user-profile raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "rename-user-profile/renames-user-profile-a: rename-user-profile was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta rename-user-profile produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn rename_user_profile_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("rename-user-profile/renames-user-profile-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("rename-user-profile/renames-user-profile-a: committed diff decodes");
    assert_eq!(produced, committed, "rename-user-profile/renames-user-profile-a: the diff rename-user-profile builds differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn rename_user_profile_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("rename-user-profile/renames-user-profile-a: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("rename-user-profile/renames-user-profile-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("rename-user-profile/renames-user-profile-a: committed diff reparses");
    assert_eq!(reencoded, original, "rename-user-profile/renames-user-profile-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what rename-user-profile does, not a summary.
#[semio_framework_async_macros::async_test]
async fn rename_user_profile_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("rename-user-profile/renames-user-profile-a: committed diff decodes");
    let produced = decoded.apply(&before()).expect("rename-user-profile/renames-user-profile-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-user-profile/renames-user-profile-a: the committed diff did not carry before to after");
}
