//! 🧪️ `change-m-ed-knm` fixture — `raises-the-design-bending-moment-to-12-5-knm`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-m-ed-knm` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1996::{En1996Diff, En1996Mutation, En1996Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1996Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1996Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1996Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-m-ed-knm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising M_Ed from 8.0 kNm to 12.5 kNm rewrites `m_ed_knm` and nothing else — the section modulus `z_mm3`
/// that the EN 1996-1-1 §6.3 flexure check pairs it with is untouched.
#[semio_framework_async_macros::async_test]
fn raises_the_design_bending_moment_to_12_5_knm() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-m-ed-knm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.m_ed_knm, 12.5, "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: m_ed_knm must read 12.5 kNm once the change lands");
    assert_eq!(applied.z_mm3, before().z_mm3, "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: the section modulus is the resistance side of the same flexure check and must not move with the action");
}

/// ↩️ `change-m-ed-knm`'s inverse reads the OLD 8.0 kNm out of BASE, so replaying it puts the 8.0 kNm back on
/// `m_ed_knm`.
#[semio_framework_async_macros::async_test]
fn restoring_8_knm_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-m-ed-knm applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: the inverse of one change-m-ed-knm is exactly one change-m-ed-knm back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-m-ed-knm inverse step applies");
    }
    assert_eq!(snapshot.m_ed_knm, base.m_ed_knm, "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: the inverse must put the 8.0 kNm back on `m_ed_knm`");
    assert_eq!(snapshot, base, "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-m-ed-knm` payload are already canonical: decode →
/// encode is a fixed point, so `newMEdKnm` (serde camelCase over `new_m_ed_knm`) is spelled here exactly as
/// this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-m-ed-knm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-m-ed-knm payload reparses");
    assert_eq!(reencoded, original, "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: the committed change-m-ed-knm JSON is not canonical");
}

/// 🎯️ 12.5 kNm is finite and differs from the committed 8.0 kNm, so neither the `mutation.invariant`
/// fatal nor the `mutation.no-op` warning in `change-m-ed-knm`'s diff builder can fire.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: the payload is a finite number, so the `is_finite` fatal guard stays shut, and 12.5 differs from the committed 8.0, so the `mutation.no-op` warning guard stays shut too"
    );
    assert!(produced.messages().is_empty(), "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: an accepted change-m-ed-knm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-m-ed-knm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `mEdKnm` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-m-ed-knm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the design bending moment
/// and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-m-ed-knm diff decodes");
    assert_eq!(decoded.m_ed_knm, Some(12.5), "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: the committed diff must carry mEdKnm = 12.5 kNm");
    assert!(decoded.z_mm3.is_none(), "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: change-m-ed-knm writes mEdKnm and must leave `z_mm3` untouched");
    assert!(decoded.n_ed_kn.is_none(), "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: change-m-ed-knm writes mEdKnm and must leave `n_ed_kn` untouched");
    assert!(decoded.artifact.is_none(), "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the moment change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-m-ed-knm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: the committed diff did not carry before to after");
    assert_eq!(produced.m_ed_knm, 12.5, "change-m-ed-knm/raises-the-design-bending-moment-to-12-5-knm: applying the committed diff must land m_ed_knm on 12.5 kNm");
}
