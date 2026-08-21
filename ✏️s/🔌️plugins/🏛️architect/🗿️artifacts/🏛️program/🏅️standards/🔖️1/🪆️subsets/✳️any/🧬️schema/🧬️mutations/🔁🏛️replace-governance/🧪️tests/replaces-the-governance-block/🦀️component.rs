//! 🧪️ `replace-governance` fixture — `replaces-the-governance-block`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️component.rs`, which emits the payload value as the whole `governance` facet verbatim.
//!
//! That leaf's own contract line reads: 🔁️ New `Governance` wholesale. Root-scoped singleton — always present, so Warning `mutation.no-op` (empty diff) covers the only degenerate case: the value is unchanged.
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
    serde_json::from_str(BEFORE).expect("replace-governance/replaces-the-governance-block: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("replace-governance/replaces-the-governance-block: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("replace-governance/replaces-the-governance-block: mutation decodes")
}

/// ▶️ replace-governance carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn replace_governance_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("replace-governance/replaces-the-governance-block: replace-governance applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-governance/replaces-the-governance-block: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying replace-governance and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn replace_governance_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("replace-governance/replaces-the-governance-block: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("replace-governance/replaces-the-governance-block: inverse step applies");
    }
    assert_eq!(state, base, "replace-governance/replaces-the-governance-block: replace-governance back to the captured prior value did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed replace-governance payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn replace_governance_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("replace-governance/replaces-the-governance-block: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("replace-governance/replaces-the-governance-block: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("replace-governance/replaces-the-governance-block: snapshot reparses");
        assert_eq!(reencoded, original, "replace-governance/replaces-the-governance-block: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("replace-governance/replaces-the-governance-block: replace-governance payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("replace-governance/replaces-the-governance-block: replace-governance payload reparses");
    assert_eq!(reencoded, original, "replace-governance/replaces-the-governance-block: committed replace-governance payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: replace-governance applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn replace_governance_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("replace-governance/replaces-the-governance-block: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-governance/replaces-the-governance-block: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "replace-governance/replaces-the-governance-block: replace-governance raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "replace-governance/replaces-the-governance-block: replace-governance was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta replace-governance produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn replace_governance_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("replace-governance/replaces-the-governance-block: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("replace-governance/replaces-the-governance-block: committed diff decodes");
    assert_eq!(produced, committed, "replace-governance/replaces-the-governance-block: the diff replace-governance builds differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn replace_governance_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("replace-governance/replaces-the-governance-block: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("replace-governance/replaces-the-governance-block: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("replace-governance/replaces-the-governance-block: committed diff reparses");
    assert_eq!(reencoded, original, "replace-governance/replaces-the-governance-block: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what replace-governance does, not a summary.
#[semio_framework_async_macros::async_test]
async fn replace_governance_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("replace-governance/replaces-the-governance-block: committed diff decodes");
    let produced = decoded.apply(&before()).expect("replace-governance/replaces-the-governance-block: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-governance/replaces-the-governance-block: the committed diff did not carry before to after");
}
