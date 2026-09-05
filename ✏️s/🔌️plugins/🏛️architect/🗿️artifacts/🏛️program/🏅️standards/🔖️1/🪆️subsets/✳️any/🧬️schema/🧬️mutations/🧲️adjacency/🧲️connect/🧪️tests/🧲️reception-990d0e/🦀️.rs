//! 🧪️ `connect-adjacency` fixture — `🧲️reception-waiting`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which checks both endpoint elements exist, normalizes the pair, forces `normalized = true`, and — because no edge yet joins `element-a`/`element-b` — takes the `added = [normalized edge]` branch.
//!
//! That leaf's own contract line reads: 🔌️ Error `mutation.target-missing` if either endpoint element is absent (empty diff); Warning `mutation.no-op` if the edge already carries this exact value (empty diff); else `added = [normalized edge]` if the pair is new, else `patched = [{existing id, full patch}]` — the existing edge's own id is preserved even if `payload.adjacency` carries a different one.
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
    serde_json::from_str(BEFORE).expect("connect-adjacency/connects-reception-to-waiting: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("connect-adjacency/connects-reception-to-waiting: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("connect-adjacency/connects-reception-to-waiting: mutation decodes")
}

/// ▶️ connect-adjacency carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn connect_adjacency_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("connect-adjacency/connects-reception-to-waiting: connect-adjacency applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "connect-adjacency/connects-reception-to-waiting: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying connect-adjacency and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn connect_adjacency_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("connect-adjacency/connects-reception-to-waiting: forward diff applies");
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("connect-adjacency/connects-reception-to-waiting: inverse step applies");
    }
    assert_eq!(state, base, "connect-adjacency/connects-reception-to-waiting: disconnect-adjacency (the inverse of a pair-creating connect) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed connect-adjacency payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn connect_adjacency_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("connect-adjacency/connects-reception-to-waiting: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("connect-adjacency/connects-reception-to-waiting: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("connect-adjacency/connects-reception-to-waiting: snapshot reparses");
        assert_eq!(reencoded, original, "connect-adjacency/connects-reception-to-waiting: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("connect-adjacency/connects-reception-to-waiting: connect-adjacency payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("connect-adjacency/connects-reception-to-waiting: connect-adjacency payload reparses");
    assert_eq!(reencoded, original, "connect-adjacency/connects-reception-to-waiting: committed connect-adjacency payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: connect-adjacency applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn connect_adjacency_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("connect-adjacency/connects-reception-to-waiting: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "connect-adjacency/connects-reception-to-waiting: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "connect-adjacency/connects-reception-to-waiting: connect-adjacency raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "connect-adjacency/connects-reception-to-waiting: connect-adjacency was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta connect-adjacency produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn connect_adjacency_produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("connect-adjacency/connects-reception-to-waiting: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("connect-adjacency/connects-reception-to-waiting: committed diff decodes");
    assert_eq!(produced, committed, "connect-adjacency/connects-reception-to-waiting: the diff connect-adjacency builds differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn connect_adjacency_committed_diff_is_canonical() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("connect-adjacency/connects-reception-to-waiting: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("connect-adjacency/connects-reception-to-waiting: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("connect-adjacency/connects-reception-to-waiting: committed diff reparses");
    assert_eq!(reencoded, original, "connect-adjacency/connects-reception-to-waiting: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what connect-adjacency does, not a summary.
#[semio_framework_async_macros::async_test]
async fn connect_adjacency_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("connect-adjacency/connects-reception-to-waiting: committed diff decodes");
    let produced = decoded.apply(&before()).expect("connect-adjacency/connects-reception-to-waiting: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "connect-adjacency/connects-reception-to-waiting: the committed diff did not carry before to after");
}
