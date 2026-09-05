//! 🧪️ `change-fire-duration-min` fixture — `🟨️raises-the-fire-exposure-from-r30-to-r60`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-fire-duration-min` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1995::{En1995Diff, En1995Mutation, En1995Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1995Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1995Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1995Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-fire-duration-min` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising the fire exposure from R30 to R60 rewrites `fire_duration_min` alone. The EN 1995-1-2 §4.2
/// charring depth doubles from 21.0 mm to 42.0 mm at β_n = 0.7 mm/min, but the residual section is COMPUTED,
/// so b and h must ride through unchanged.
#[semio_framework_async_macros::async_test]
fn raises_the_fire_exposure_from_r30_to_r60() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-fire-duration-min applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.fire_duration_min, 60.0, "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: fire_duration_min must read 60.0 minutes once the change lands");
    assert_eq!(applied.b_mm, before().b_mm, "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: the declared beam width is the INITIAL section; charring reduction belongs to the fire check, never to the document");
}

/// ↩️ `change-fire-duration-min`'s inverse reads the OLD 30.0 minutes out of BASE, so replaying it puts the R30
/// exposure back on `fire_duration_min`.
#[semio_framework_async_macros::async_test]
fn returning_to_r30_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-fire-duration-min applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: the inverse of one change-fire-duration-min is exactly one change-fire-duration-min back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-fire-duration-min inverse step applies");
    }
    assert_eq!(snapshot.fire_duration_min, base.fire_duration_min, "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: the inverse must put the R30 exposure back on `fire_duration_min`");
    assert_eq!(snapshot, base, "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-fire-duration-min` payload are already canonical:
/// decode → encode is a fixed point, so `{"ChangeFireDurationMin": {"newFireDurationMin": 60.0}}` — a JSON
/// FLOAT, because the field is an `f64` is spelled here exactly as this artifact's own serde attributes
/// render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-fire-duration-min payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-fire-duration-min payload reparses");
    assert_eq!(reencoded, original, "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: the committed change-fire-duration-min JSON is not canonical");
}

/// 🎯️ `fire_duration_min` is an `f64` here (unlike en1996's `u32` fire field), so
/// `change-fire-duration-min` DOES carry a finiteness guard; 60.0 is finite and differs from 30.0.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: the payload is finite, so `change-fire-duration-min`'s `mutation.invariant` fatal cannot fire, and 60.0 differs from the committed 30.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: an accepted change-fire-duration-min emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-fire-duration-min` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `fireDurationMin` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-fire-duration-min diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the fire exposure duration
/// and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-fire-duration-min diff decodes");
    assert_eq!(decoded.fire_duration_min, Some(60.0), "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: the committed diff must carry fireDurationMin = 60.0 minutes");
    assert!(decoded.b_mm.is_none(), "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: change-fire-duration-min writes fireDurationMin and must leave `b_mm` untouched");
    assert!(decoded.h_mm.is_none(), "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: change-fire-duration-min writes fireDurationMin and must leave `h_mm` untouched");
    assert!(decoded.artifact.is_none(), "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the fire-exposure change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-fire-duration-min diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: the committed diff did not carry before to after");
    assert_eq!(produced.fire_duration_min, 60.0, "change-fire-duration-min/raises-the-fire-exposure-from-r30-to-r60: applying the committed diff must land fire_duration_min on 60.0 minutes");
}
