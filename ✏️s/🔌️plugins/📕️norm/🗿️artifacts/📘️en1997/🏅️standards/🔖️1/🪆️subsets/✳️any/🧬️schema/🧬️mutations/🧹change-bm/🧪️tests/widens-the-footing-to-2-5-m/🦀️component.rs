//! 🧪️ `change-bm` fixture — `widens-the-footing-to-2-5-m`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-bm` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1997::{En1997Diff, En1997Mutation, En1997Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1997Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1997Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1997Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-bm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Widening B from 2.0 m to 2.5 m rewrites `b_m` alone. B scales the 0.5·γ·B·Nγ bearing term directly, yet
/// the declared footing AREA is a separate field and this mutation must not silently push it from 4.0 m² to
/// 6.25 m².
#[semio_framework_async_macros::async_test]
async fn widens_the_footing_to_2_5_m() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-bm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-bm/widens-the-footing-to-2-5-m: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.b_m, 2.5, "change-bm/widens-the-footing-to-2-5-m: b_m must read 2.5 m once the change lands");
    assert_eq!(applied.footing_area_m2, before().footing_area_m2, "change-bm/widens-the-footing-to-2-5-m: the footing area is declared independently of B and must not be recomputed as B² by a width edit");
}

/// ↩️ `change-bm`'s inverse reads the OLD 2.0 m out of BASE, so replaying it puts the 2.0 m footing width back
/// on `b_m`.
#[semio_framework_async_macros::async_test]
async fn restoring_2_m_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-bm applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-bm/widens-the-footing-to-2-5-m: the inverse of one change-bm is exactly one change-bm back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-bm inverse step applies");
    }
    assert_eq!(snapshot.b_m, base.b_m, "change-bm/widens-the-footing-to-2-5-m: the inverse must put the 2.0 m footing width back on `b_m`");
    assert_eq!(snapshot, base, "change-bm/widens-the-footing-to-2-5-m: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-bm` payload are already canonical: decode → encode is a
/// fixed point, so `newBM` — serde camelCase over `new_b_m` capitalises the trailing `m` segment, giving
/// `BM`, not `Bm` is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-bm/widens-the-footing-to-2-5-m: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-bm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-bm payload reparses");
    assert_eq!(reencoded, original, "change-bm/widens-the-footing-to-2-5-m: the committed change-bm JSON is not canonical");
}

/// 🎯️ 2.5 m is finite and differs from the committed 2.0 m, so `change-bm` (whose guard message
/// reads "Footing width B [m]") emits nothing.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-bm/widens-the-footing-to-2-5-m: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-bm/widens-the-footing-to-2-5-m: the payload is finite, so `change-bm`'s `mutation.invariant` fatal cannot fire, and 2.5 differs from the committed 2.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-bm/widens-the-footing-to-2-5-m: an accepted change-bm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-bm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `bM` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-bm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-bm/widens-the-footing-to-2-5-m: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the footing width and
/// nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-bm diff decodes");
    assert_eq!(decoded.b_m, Some(2.5), "change-bm/widens-the-footing-to-2-5-m: the committed diff must carry bM = 2.5 m");
    assert!(decoded.footing_area_m2.is_none(), "change-bm/widens-the-footing-to-2-5-m: change-bm writes bM and must leave `footing_area_m2` untouched");
    assert!(decoded.d_f_m.is_none(), "change-bm/widens-the-footing-to-2-5-m: change-bm writes bM and must leave `d_f_m` untouched");
    assert!(decoded.artifact.is_none(), "change-bm/widens-the-footing-to-2-5-m: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-bm/widens-the-footing-to-2-5-m: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the footing-width change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-bm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-bm/widens-the-footing-to-2-5-m: the committed diff did not carry before to after");
    assert_eq!(produced.b_m, 2.5, "change-bm/widens-the-footing-to-2-5-m: applying the committed diff must land b_m on 2.5 m");
}
