//! 🧪️ `change-h-mm` fixture — `deepens-the-beam-to-360-mm`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-h-mm` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-h-mm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Deepening the beam from 300.0 mm to 360.0 mm rewrites `h_mm` alone. `section_depth_mm` — the depth the EN
/// 1995-1-1 §3.2 size-effect factor k_h reads — is a SEPARATE field that happens to start equal, and it must
/// not be dragged along.
#[semio_framework_async_macros::async_test]
async fn deepens_the_beam_to_360_mm() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-h-mm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-h-mm/deepens-the-beam-to-360-mm: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.h_mm, 360.0, "change-h-mm/deepens-the-beam-to-360-mm: h_mm must read 360.0 mm once the change lands");
    assert_eq!(applied.section_depth_mm, before().section_depth_mm, "change-h-mm/deepens-the-beam-to-360-mm: the k_h size-effect depth is its own field and coincides with h only in the committed base");
}

/// ↩️ `change-h-mm`'s inverse reads the OLD 300.0 mm out of BASE, so replaying it puts the 300.0 mm depth back
/// on `h_mm`.
#[semio_framework_async_macros::async_test]
async fn restoring_300_mm_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-h-mm applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-h-mm/deepens-the-beam-to-360-mm: the inverse of one change-h-mm is exactly one change-h-mm back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-h-mm inverse step applies");
    }
    assert_eq!(snapshot.h_mm, base.h_mm, "change-h-mm/deepens-the-beam-to-360-mm: the inverse must put the 300.0 mm depth back on `h_mm`");
    assert_eq!(snapshot, base, "change-h-mm/deepens-the-beam-to-360-mm: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-h-mm` payload are already canonical: decode → encode is
/// a fixed point, so `{"ChangeHMm": {"newHMm": 360.0}}` — externally tagged is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-h-mm/deepens-the-beam-to-360-mm: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-h-mm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-h-mm payload reparses");
    assert_eq!(reencoded, original, "change-h-mm/deepens-the-beam-to-360-mm: the committed change-h-mm JSON is not canonical");
}

/// 🎯️ 360.0 mm is finite and differs from the committed 300.0 mm, so `change-h-mm` produces a
/// message-free outcome.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-h-mm/deepens-the-beam-to-360-mm: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-h-mm/deepens-the-beam-to-360-mm: the payload is finite, so `change-h-mm`'s `mutation.invariant` fatal cannot fire, and 360.0 differs from the committed 300.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-h-mm/deepens-the-beam-to-360-mm: an accepted change-h-mm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-h-mm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `hMm` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-h-mm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-h-mm/deepens-the-beam-to-360-mm: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the beam depth and nothing
/// else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-h-mm diff decodes");
    assert_eq!(decoded.h_mm, Some(360.0), "change-h-mm/deepens-the-beam-to-360-mm: the committed diff must carry hMm = 360.0 mm");
    assert!(decoded.section_depth_mm.is_none(), "change-h-mm/deepens-the-beam-to-360-mm: change-h-mm writes hMm and must leave `section_depth_mm` untouched");
    assert!(decoded.w_mm3.is_none(), "change-h-mm/deepens-the-beam-to-360-mm: change-h-mm writes hMm and must leave `w_mm3` untouched");
    assert!(decoded.artifact.is_none(), "change-h-mm/deepens-the-beam-to-360-mm: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-h-mm/deepens-the-beam-to-360-mm: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the depth change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-h-mm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-h-mm/deepens-the-beam-to-360-mm: the committed diff did not carry before to after");
    assert_eq!(produced.h_mm, 360.0, "change-h-mm/deepens-the-beam-to-360-mm: applying the committed diff must land h_mm on 360.0 mm");
}
