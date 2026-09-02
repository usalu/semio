//! 🧪️ `change-section-depth-mm` fixture — `raises-the-size-effect-depth-to-360-mm`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-section-depth-mm` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-section-depth-mm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising the k_h size-effect depth from 300.0 mm to 360.0 mm rewrites `section_depth_mm` alone. It is the
/// mirror image of the `change-h-mm` case: the geometric depth `h_mm` starts equal to it and must NOT follow.
#[semio_framework_async_macros::async_test]
fn raises_the_size_effect_depth_to_360_mm() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-section-depth-mm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.section_depth_mm, 360.0, "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: section_depth_mm must read 360.0 mm once the change lands");
    assert_eq!(applied.h_mm, before().h_mm, "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: the geometric beam depth is a separate declared field that coincides with the k_h depth only in the committed base");
}

/// ↩️ `change-section-depth-mm`'s inverse reads the OLD 300.0 mm out of BASE, so replaying it puts the 300.0 mm
/// size-effect depth back on `section_depth_mm`.
#[semio_framework_async_macros::async_test]
fn restoring_the_300_mm_size_effect_depth_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-section-depth-mm applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: the inverse of one change-section-depth-mm is exactly one change-section-depth-mm back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-section-depth-mm inverse step applies");
    }
    assert_eq!(snapshot.section_depth_mm, base.section_depth_mm, "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: the inverse must put the 300.0 mm size-effect depth back on `section_depth_mm`");
    assert_eq!(snapshot, base, "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-section-depth-mm` payload are already canonical: decode
/// → encode is a fixed point, so `{"ChangeSectionDepthMm": {"newSectionDepthMm": 360.0}}` — externally tagged
/// is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-section-depth-mm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-section-depth-mm payload reparses");
    assert_eq!(reencoded, original, "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: the committed change-section-depth-mm JSON is not canonical");
}

/// 🎯️ 360.0 mm is finite and differs from the committed 300.0 mm, so `change-section-depth-mm`
/// emits nothing.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: the payload is finite, so `change-section-depth-mm`'s `mutation.invariant` fatal cannot fire, and 360.0 differs from the committed 300.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: an accepted change-section-depth-mm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-section-depth-mm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `sectionDepthMm` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-section-depth-mm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the size-effect depth and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-section-depth-mm diff decodes");
    assert_eq!(decoded.section_depth_mm, Some(360.0), "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: the committed diff must carry sectionDepthMm = 360.0 mm");
    assert!(decoded.h_mm.is_none(), "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: change-section-depth-mm writes sectionDepthMm and must leave `h_mm` untouched");
    assert!(decoded.f_m_k.is_none(), "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: change-section-depth-mm writes sectionDepthMm and must leave `f_m_k` untouched");
    assert!(decoded.artifact.is_none(), "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the size-effect-depth change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-section-depth-mm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: the committed diff did not carry before to after");
    assert_eq!(produced.section_depth_mm, 360.0, "change-section-depth-mm/raises-the-size-effect-depth-to-360-mm: applying the committed diff must land section_depth_mm on 360.0 mm");
}
