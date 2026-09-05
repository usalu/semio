//! 🧪️ `change-storeys` fixture — `🏢️adds-a-third-storey-at-the-simplified-method-limit`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-storeys` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1996::{En1996Diff, En1996Mutation, En1996Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1996Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1996Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1996Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-storeys` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Going from 2 to 3 storeys rewrites `storeys` alone. Three is exactly the EN 1996-3 §1.1 scope limit, so
/// the simplified method stays applicable — and the effective height, which the other half of that scope test
/// reads, must not be touched.
#[semio_framework_async_macros::async_test]
fn adds_a_third_storey_at_the_simplified_method_limit() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-storeys applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.storeys, 3, "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: storeys must read 3 storeys once the change lands");
    assert_eq!(applied.h_ef_mm, before().h_ef_mm, "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: the effective height feeds the slenderness half of the same EN 1996-3 applicability test and must not move with the storey count");
}

/// ↩️ `change-storeys`'s inverse reads the OLD 2 out of BASE, so replaying it puts the two-storey count back on
/// `storeys`.
#[semio_framework_async_macros::async_test]
fn returning_to_two_storeys_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-storeys applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: the inverse of one change-storeys is exactly one change-storeys back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-storeys inverse step applies");
    }
    assert_eq!(snapshot.storeys, base.storeys, "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: the inverse must put the two-storey count back on `storeys`");
    assert_eq!(snapshot, base, "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-storeys` payload are already canonical: decode → encode
/// is a fixed point, so `newStoreys`, carrying a bare JSON integer because the field is a `u32` is spelled
/// here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-storeys payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-storeys payload reparses");
    assert_eq!(reencoded, original, "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: the committed change-storeys JSON is not canonical");
}

/// 🎯️ `storeys` is a `u32`, so `change-storeys` has no finiteness guard; 3 differs from the
/// committed 2, so no `mutation.no-op` warning is raised.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: `change-storeys` has no numeric-finiteness guard at all — only the equality guard — and 3 differs from the committed committed 2, so `mutation.no-op` must not fire"
    );
    assert!(produced.messages().is_empty(), "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: an accepted change-storeys emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-storeys` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `storeys` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-storeys diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the storey count and nothing
/// else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-storeys diff decodes");
    assert_eq!(decoded.storeys, Some(3), "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: the committed diff must carry storeys = 3 storeys");
    assert!(decoded.h_ef_mm.is_none(), "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: change-storeys writes storeys and must leave `h_ef_mm` untouched");
    assert!(decoded.t_ef_mm.is_none(), "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: change-storeys writes storeys and must leave `t_ef_mm` untouched");
    assert!(decoded.artifact.is_none(), "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the storey-count change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-storeys diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: the committed diff did not carry before to after");
    assert_eq!(produced.storeys, 3, "change-storeys/adds-a-third-storey-at-the-simplified-method-limit: applying the committed diff must land storeys on 3 storeys");
}
