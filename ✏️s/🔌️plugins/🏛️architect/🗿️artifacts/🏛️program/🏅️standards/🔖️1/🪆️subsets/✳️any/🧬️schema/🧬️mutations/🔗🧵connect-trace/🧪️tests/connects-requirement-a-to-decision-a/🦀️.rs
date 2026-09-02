//! 🧪️ `connect-trace` fixture — `connects-requirement-a-to-decision-a`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which finds no trace with this id and therefore takes the `added = [trace]` branch (endpoints are free-form, unchecked).
//!
//! That leaf's own contract line reads: 🔌️ Warning `mutation.no-op` if the trace already carries this exact value (empty diff); else `added = [trace]` if the id is new, else `patched = [{id, full patch}]`. `from_id`/`to_id` are free-form cross-register references (any entity across any collection) — endpoint-existence checking is not implemented here; see `📓️w3-d-architect-report.md`.
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
    serde_json::from_str(BEFORE).expect("connect-trace/connects-requirement-a-to-decision-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("connect-trace/connects-requirement-a-to-decision-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("connect-trace/connects-requirement-a-to-decision-a: mutation decodes")
}

/// ▶️ connect-trace carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn connect_trace_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("connect-trace/connects-requirement-a-to-decision-a: connect-trace applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "connect-trace/connects-requirement-a-to-decision-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying connect-trace and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn connect_trace_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("connect-trace/connects-requirement-a-to-decision-a: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("connect-trace/connects-requirement-a-to-decision-a: inverse step applies");
    }
    assert_eq!(state, base, "connect-trace/connects-requirement-a-to-decision-a: disconnect-trace (the inverse of an id-creating connect) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed connect-trace payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn connect_trace_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("connect-trace/connects-requirement-a-to-decision-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("connect-trace/connects-requirement-a-to-decision-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("connect-trace/connects-requirement-a-to-decision-a: snapshot reparses");
        assert_eq!(reencoded, original, "connect-trace/connects-requirement-a-to-decision-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("connect-trace/connects-requirement-a-to-decision-a: connect-trace payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("connect-trace/connects-requirement-a-to-decision-a: connect-trace payload reparses");
    assert_eq!(reencoded, original, "connect-trace/connects-requirement-a-to-decision-a: committed connect-trace payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: connect-trace applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn connect_trace_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("connect-trace/connects-requirement-a-to-decision-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "connect-trace/connects-requirement-a-to-decision-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "connect-trace/connects-requirement-a-to-decision-a: connect-trace raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "connect-trace/connects-requirement-a-to-decision-a: connect-trace was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta connect-trace produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn connect_trace_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("connect-trace/connects-requirement-a-to-decision-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("connect-trace/connects-requirement-a-to-decision-a: committed diff decodes");
    assert_eq!(produced, committed, "connect-trace/connects-requirement-a-to-decision-a: the diff connect-trace builds differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn connect_trace_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("connect-trace/connects-requirement-a-to-decision-a: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("connect-trace/connects-requirement-a-to-decision-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("connect-trace/connects-requirement-a-to-decision-a: committed diff reparses");
    assert_eq!(reencoded, original, "connect-trace/connects-requirement-a-to-decision-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what connect-trace does, not a summary.
#[semio_framework_async_macros::async_test]
async fn connect_trace_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("connect-trace/connects-requirement-a-to-decision-a: committed diff decodes");
    let produced = decoded.apply(&before()).expect("connect-trace/connects-requirement-a-to-decision-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "connect-trace/connects-requirement-a-to-decision-a: the committed diff did not carry before to after");
}
