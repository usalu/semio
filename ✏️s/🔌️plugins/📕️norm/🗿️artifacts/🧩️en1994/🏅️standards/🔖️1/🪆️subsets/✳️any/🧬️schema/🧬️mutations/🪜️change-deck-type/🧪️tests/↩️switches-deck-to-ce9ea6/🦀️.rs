//! 🧪️ `change-deck-type` fixture — `↩️switches-deck-to-re-entrant` (EN 1994 composite).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1994Snapshot {
    serde_json::from_str(BEFORE).expect("change-deck-type/switches-deck-to-re-entrant: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-deck-type/switches-deck-to-re-entrant: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-deck-type/switches-deck-to-re-entrant: mutation decodes")
}

/// ▶️ `change-deck-type` carries `deck_type` from trapezoidal to re_entrant and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-deck-type/switches-deck-to-re-entrant: mutation applies to its committed before-snapshot");
    assert_eq!(produced.deck_type, "re_entrant", "change-deck-type/switches-deck-to-re-entrant: `deck_type` must read re_entrant after the mutation");
    assert_eq!(produced.delta_sigma_mpa, base.delta_sigma_mpa, "change-deck-type/switches-deck-to-re-entrant: `delta_sigma_mpa` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-deck-type/switches-deck-to-re-entrant: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `deck_type` (trapezoidal) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-deck-type/switches-deck-to-re-entrant: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-deck-type/switches-deck-to-re-entrant: inverse step applies");
    }
    assert_eq!(snapshot.deck_type, base.deck_type, "change-deck-type/switches-deck-to-re-entrant: inverse must put `deck_type` back to trapezoidal");
    assert_eq!(snapshot, base, "change-deck-type/switches-deck-to-re-entrant: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-deck-type/switches-deck-to-re-entrant: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-deck-type/switches-deck-to-re-entrant: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-deck-type/switches-deck-to-re-entrant: snapshot reparses");
        assert_eq!(reencoded, original, "change-deck-type/switches-deck-to-re-entrant: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-deck-type/switches-deck-to-re-entrant: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-deck-type/switches-deck-to-re-entrant: mutation reparses");
    assert_eq!(reencoded, original, "change-deck-type/switches-deck-to-re-entrant: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean trapezoidal→re_entrant edit of `deck_type` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-deck-type/switches-deck-to-re-entrant: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-deck-type/switches-deck-to-re-entrant: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-deck-type/switches-deck-to-re-entrant: changing `deck_type` away from trapezoidal must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-deck-type/switches-deck-to-re-entrant: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `deckType` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().deck_type.as_deref(), Some("re_entrant"), "change-deck-type/switches-deck-to-re-entrant: the diff must carry `deck_type` = re_entrant");
    assert!(outcome.diff().delta_sigma_mpa.is_none(), "change-deck-type/switches-deck-to-re-entrant: the diff must leave `delta_sigma_mpa` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-deck-type/switches-deck-to-re-entrant: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-deck-type/switches-deck-to-re-entrant: committed diff decodes");
    assert_eq!(produced, committed, "change-deck-type/switches-deck-to-re-entrant: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-deck-type/switches-deck-to-re-entrant: committed diff decodes");
    assert_eq!(decoded.deck_type.as_deref(), Some("re_entrant"), "change-deck-type/switches-deck-to-re-entrant: the committed diff must name `deck_type` = re_entrant");
    let reencoded = serde_json::to_value(&decoded).expect("change-deck-type/switches-deck-to-re-entrant: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-deck-type/switches-deck-to-re-entrant: committed diff reparses");
    assert_eq!(reencoded, original, "change-deck-type/switches-deck-to-re-entrant: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the re_entrant `deck_type` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-deck-type/switches-deck-to-re-entrant: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-deck-type/switches-deck-to-re-entrant: committed diff applies to the before-snapshot");
    assert_eq!(produced.deck_type, "re_entrant", "change-deck-type/switches-deck-to-re-entrant: the committed diff must set `deck_type` to re_entrant");
    assert_eq!(produced, expected_after(), "change-deck-type/switches-deck-to-re-entrant: committed diff did not carry before to after");
}
