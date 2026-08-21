//! 🧪️ `change-b-mm` fixture — `widens-the-beam-to-240-mm`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-b-mm` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1995::{En1995Diff, En1995Mutation, En1995Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1995Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1995Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1995Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-b-mm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Widening the beam from 200.0 mm to 240.0 mm rewrites `b_mm` alone. Neither the gross area nor the section
/// modulus is recomputed — en1995 stores b, h, A and W as four independent declared inputs, and this fixture
/// is the pin on that.
#[semio_framework_async_macros::async_test]
async fn widens_the_beam_to_240_mm() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-b-mm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-b-mm/widens-the-beam-to-240-mm: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.b_mm, 240.0, "change-b-mm/widens-the-beam-to-240-mm: b_mm must read 240.0 mm once the change lands");
    assert_eq!(applied.a_mm2, before().a_mm2, "change-b-mm/widens-the-beam-to-240-mm: the gross area is declared independently and must not be recomputed as b·h by a width edit");
}

/// ↩️ `change-b-mm`'s inverse reads the OLD 200.0 mm out of BASE, so replaying it puts the 200.0 mm width back
/// on `b_mm`.
#[semio_framework_async_macros::async_test]
async fn restoring_200_mm_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-b-mm applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-b-mm/widens-the-beam-to-240-mm: the inverse of one change-b-mm is exactly one change-b-mm back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-b-mm inverse step applies");
    }
    assert_eq!(snapshot.b_mm, base.b_mm, "change-b-mm/widens-the-beam-to-240-mm: the inverse must put the 200.0 mm width back on `b_mm`");
    assert_eq!(snapshot, base, "change-b-mm/widens-the-beam-to-240-mm: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-b-mm` payload are already canonical: decode → encode is
/// a fixed point, so `{"ChangeBMm": {"newBMm": 240.0}}` — externally tagged is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-b-mm/widens-the-beam-to-240-mm: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-b-mm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-b-mm payload reparses");
    assert_eq!(reencoded, original, "change-b-mm/widens-the-beam-to-240-mm: the committed change-b-mm JSON is not canonical");
}

/// 🎯️ 240.0 mm is finite and differs from the committed 200.0 mm, so `change-b-mm` stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-b-mm/widens-the-beam-to-240-mm: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-b-mm/widens-the-beam-to-240-mm: the payload is finite, so `change-b-mm`'s `mutation.invariant` fatal cannot fire, and 240.0 differs from the committed 200.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-b-mm/widens-the-beam-to-240-mm: an accepted change-b-mm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-b-mm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `bMm` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-b-mm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-b-mm/widens-the-beam-to-240-mm: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the beam width and nothing
/// else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-b-mm diff decodes");
    assert_eq!(decoded.b_mm, Some(240.0), "change-b-mm/widens-the-beam-to-240-mm: the committed diff must carry bMm = 240.0 mm");
    assert!(decoded.a_mm2.is_none(), "change-b-mm/widens-the-beam-to-240-mm: change-b-mm writes bMm and must leave `a_mm2` untouched");
    assert!(decoded.w_mm3.is_none(), "change-b-mm/widens-the-beam-to-240-mm: change-b-mm writes bMm and must leave `w_mm3` untouched");
    assert!(decoded.artifact.is_none(), "change-b-mm/widens-the-beam-to-240-mm: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-b-mm/widens-the-beam-to-240-mm: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the width change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-b-mm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-b-mm/widens-the-beam-to-240-mm: the committed diff did not carry before to after");
    assert_eq!(produced.b_mm, 240.0, "change-b-mm/widens-the-beam-to-240-mm: applying the committed diff must land b_mm on 240.0 mm");
}
