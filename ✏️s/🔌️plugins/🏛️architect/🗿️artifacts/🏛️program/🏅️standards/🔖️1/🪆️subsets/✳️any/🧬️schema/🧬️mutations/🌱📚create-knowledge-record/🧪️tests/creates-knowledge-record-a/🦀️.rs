//! 🧪️ `create-knowledge-record` fixture — `creates-knowledge-record-a`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which reads the live `knowledge` rows off the working-scene cache, appends the payload row, and re-mints a content-addressed `table` child handle whose `childId` is a hash of the row list's JSON.
//!
//! That leaf's own contract line reads: 🌱️ Reads the live `knowledge` rows off the working-scene cache; Fatal `mutation.duplicate-id` if the id already exists (empty diff); else appends the payload row and re-mints a fresh content-addressed `table` child handle — composed-child equivalent of the former `added = [payload row]` sparse delta (`📓️migration-recipe.md` §3/§4).
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
    serde_json::from_str(BEFORE).expect("create-knowledge-record/creates-knowledge-record-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("create-knowledge-record/creates-knowledge-record-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("create-knowledge-record/creates-knowledge-record-a: mutation decodes")
}

/// ▶️ create-knowledge-record carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn create_knowledge_record_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("create-knowledge-record/creates-knowledge-record-a: create-knowledge-record applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-knowledge-record/creates-knowledge-record-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying create-knowledge-record and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn create_knowledge_record_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("create-knowledge-record/creates-knowledge-record-a: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("create-knowledge-record/creates-knowledge-record-a: inverse step applies");
    }
    assert_eq!(state, base, "create-knowledge-record/creates-knowledge-record-a: delete-knowledge-record (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed create-knowledge-record payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn create_knowledge_record_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("create-knowledge-record/creates-knowledge-record-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("create-knowledge-record/creates-knowledge-record-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("create-knowledge-record/creates-knowledge-record-a: snapshot reparses");
        assert_eq!(reencoded, original, "create-knowledge-record/creates-knowledge-record-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-knowledge-record/creates-knowledge-record-a: create-knowledge-record payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-knowledge-record/creates-knowledge-record-a: create-knowledge-record payload reparses");
    assert_eq!(reencoded, original, "create-knowledge-record/creates-knowledge-record-a: committed create-knowledge-record payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: create-knowledge-record applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn create_knowledge_record_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("create-knowledge-record/creates-knowledge-record-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-knowledge-record/creates-knowledge-record-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "create-knowledge-record/creates-knowledge-record-a: create-knowledge-record raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "create-knowledge-record/creates-knowledge-record-a: create-knowledge-record was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta create-knowledge-record produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn create_knowledge_record_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("create-knowledge-record/creates-knowledge-record-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("create-knowledge-record/creates-knowledge-record-a: committed diff decodes");
    assert_eq!(produced, committed, "create-knowledge-record/creates-knowledge-record-a: the diff create-knowledge-record builds differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn create_knowledge_record_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("create-knowledge-record/creates-knowledge-record-a: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("create-knowledge-record/creates-knowledge-record-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("create-knowledge-record/creates-knowledge-record-a: committed diff reparses");
    assert_eq!(reencoded, original, "create-knowledge-record/creates-knowledge-record-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what create-knowledge-record does, not a summary.
#[semio_framework_async_macros::async_test]
async fn create_knowledge_record_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("create-knowledge-record/creates-knowledge-record-a: committed diff decodes");
    let produced = decoded.apply(&before()).expect("create-knowledge-record/creates-knowledge-record-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-knowledge-record/creates-knowledge-record-a: the committed diff did not carry before to after");
}
