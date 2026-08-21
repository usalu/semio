//! 🧪️ `replace-site-context` fixture — `replaces-site-context-a`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️component.rs`, which patches `site-context-a` in `program.site_context` with the FULL `None` that `Patchable::diff_patch` snapshots off the payload row (this fixture moves `site_name`).
//!
//! That leaf's own contract line reads: 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
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
    serde_json::from_str(BEFORE).expect("replace-site-context/replaces-site-context-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("replace-site-context/replaces-site-context-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("replace-site-context/replaces-site-context-a: mutation decodes")
}

/// ▶️ replace-site-context carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn replace_site_context_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("replace-site-context/replaces-site-context-a: replace-site-context applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-site-context/replaces-site-context-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying replace-site-context and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn replace_site_context_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("replace-site-context/replaces-site-context-a: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("replace-site-context/replaces-site-context-a: inverse step applies");
    }
    assert_eq!(state, base, "replace-site-context/replaces-site-context-a: replace-site-context (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed replace-site-context payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn replace_site_context_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("replace-site-context/replaces-site-context-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("replace-site-context/replaces-site-context-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("replace-site-context/replaces-site-context-a: snapshot reparses");
        assert_eq!(reencoded, original, "replace-site-context/replaces-site-context-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("replace-site-context/replaces-site-context-a: replace-site-context payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("replace-site-context/replaces-site-context-a: replace-site-context payload reparses");
    assert_eq!(reencoded, original, "replace-site-context/replaces-site-context-a: committed replace-site-context payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: replace-site-context applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn replace_site_context_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("replace-site-context/replaces-site-context-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-site-context/replaces-site-context-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "replace-site-context/replaces-site-context-a: replace-site-context raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "replace-site-context/replaces-site-context-a: replace-site-context was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta replace-site-context produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn replace_site_context_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("replace-site-context/replaces-site-context-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("replace-site-context/replaces-site-context-a: committed diff decodes");
    assert_eq!(produced, committed, "replace-site-context/replaces-site-context-a: the diff replace-site-context builds differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn replace_site_context_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("replace-site-context/replaces-site-context-a: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("replace-site-context/replaces-site-context-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("replace-site-context/replaces-site-context-a: committed diff reparses");
    assert_eq!(reencoded, original, "replace-site-context/replaces-site-context-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what replace-site-context does, not a summary.
#[semio_framework_async_macros::async_test]
async fn replace_site_context_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("replace-site-context/replaces-site-context-a: committed diff decodes");
    let produced = decoded.apply(&before()).expect("replace-site-context/replaces-site-context-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-site-context/replaces-site-context-a: the committed diff did not carry before to after");
}
