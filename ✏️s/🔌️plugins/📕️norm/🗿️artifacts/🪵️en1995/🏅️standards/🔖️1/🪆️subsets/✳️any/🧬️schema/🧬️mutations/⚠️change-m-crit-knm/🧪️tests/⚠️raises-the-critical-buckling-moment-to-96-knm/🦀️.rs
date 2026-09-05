//! 🧪️ `change-m-crit-knm` fixture — `⚠️raises-the-critical-buckling-moment-to-96-knm`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-m-crit-knm` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-m-crit-knm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising M_crit from 80.0 kNm to 96.0 kNm rewrites `m_crit_knm` alone — better lateral restraint raises the
/// elastic critical moment without touching the applied design moment.
#[semio_framework_async_macros::async_test]
fn raises_the_critical_buckling_moment_to_96_knm() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-m-crit-knm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.m_crit_knm, 96.0, "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: m_crit_knm must read 96.0 kNm once the change lands");
    assert_eq!(applied.m_ed_knm, before().m_ed_knm, "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: the design moment is the action side of the §6.3.3 relative-slenderness check and is entered separately");
}

/// ↩️ `change-m-crit-knm`'s inverse reads the OLD 80.0 kNm out of BASE, so replaying it puts the 80.0 kNm back
/// on `m_crit_knm`.
#[semio_framework_async_macros::async_test]
fn restoring_80_knm_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-m-crit-knm applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: the inverse of one change-m-crit-knm is exactly one change-m-crit-knm back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-m-crit-knm inverse step applies");
    }
    assert_eq!(snapshot.m_crit_knm, base.m_crit_knm, "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: the inverse must put the 80.0 kNm back on `m_crit_knm`");
    assert_eq!(snapshot, base, "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-m-crit-knm` payload are already canonical: decode →
/// encode is a fixed point, so `{"ChangeMCritKnm": {"newMCritKnm": 96.0}}` — externally tagged is spelled
/// here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-m-crit-knm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-m-crit-knm payload reparses");
    assert_eq!(reencoded, original, "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: the committed change-m-crit-knm JSON is not canonical");
}

/// 🎯️ 96.0 kNm is finite and differs from the committed 80.0 kNm, so `change-m-crit-knm` emits
/// nothing.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: the payload is finite, so `change-m-crit-knm`'s `mutation.invariant` fatal cannot fire, and 96.0 differs from the committed 80.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: an accepted change-m-crit-knm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-m-crit-knm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `mCritKnm` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-m-crit-knm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the critical buckling moment
/// and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-m-crit-knm diff decodes");
    assert_eq!(decoded.m_crit_knm, Some(96.0), "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: the committed diff must carry mCritKnm = 96.0 kNm");
    assert!(decoded.m_ed_knm.is_none(), "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: change-m-crit-knm writes mCritKnm and must leave `m_ed_knm` untouched");
    assert!(decoded.f_m_k.is_none(), "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: change-m-crit-knm writes mCritKnm and must leave `f_m_k` untouched");
    assert!(decoded.artifact.is_none(), "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the critical-moment change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-m-crit-knm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: the committed diff did not carry before to after");
    assert_eq!(produced.m_crit_knm, 96.0, "change-m-crit-knm/raises-the-critical-buckling-moment-to-96-knm: applying the committed diff must land m_crit_knm on 96.0 kNm");
}
