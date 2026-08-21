//! 🧪️ `change-tower-m-ed-knm` fixture — `raises-tower-m-ed-knm-to-1562-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.tower_m_ed_knm` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.tower_m_ed_knm == payload.new_tower_m_ed_knm` `mutation.no-op` guard.
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from these files by `fixtures generate` and asserted by the codec matrix, never hand-forged here.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1998Snapshot {
    serde_json::from_str(BEFORE).expect("change-tower-m-ed-knm before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-tower-m-ed-knm after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-tower-m-ed-knm mutation decodes")
}

/// ▶️ `change-tower-m-ed-knm` carries the committed before-snapshot to the committed after-snapshot by moving
/// `tower_m_ed_knm` from 1200.0 to 1562.5, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_tower_m_ed_knm_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-tower-m-ed-knm applies to its committed before-snapshot");
    assert_eq!(applied.tower_m_ed_knm, 1562.5, "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: tower_m_ed_knm must read 1562.5 after the change");
    assert_eq!(applied, expected_after(), "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: a real 1200.0 to 1562.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-tower-m-ed-knm` is its own inverse partner: the inverse step restores `tower_m_ed_knm` to its pre-change
/// 1200.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_tower_m_ed_knm_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-tower-m-ed-knm applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-tower-m-ed-knm step applies");
        restored = next;
    }
    assert_eq!(restored.tower_m_ed_knm, 1200.0, "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: the inverse must put tower_m_ed_knm back to 1200.0");
    assert_eq!(restored, base, "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeTowerMEdKnm` payload are already canonical:
/// decode then encode is a fixed point, so `towerMEdKnm` and `newTowerMEdKnm` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_tower_m_ed_knm_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-tower-m-ed-knm snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-tower-m-ed-knm snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-tower-m-ed-knm snapshot reparses");
        assert_eq!(reencoded, original, "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-tower-m-ed-knm mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-tower-m-ed-knm mutation reparses");
    assert_eq!(reencoded, original, "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: committed mutation JSON is not the canonical externally-tagged ChangeTowerMEdKnm form carrying newTowerMEdKnm");
}

/// 🎯️ The declared outcome holds: `change-tower-m-ed-knm` at 1562.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_tower_m_ed_knm_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-tower-m-ed-knm outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: moving tower_m_ed_knm from 1200.0 to 1562.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-tower-m-ed-knm` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `towerMEdKnm` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `towerMRdKnm`.
#[semio_framework_async_macros::async_test]
async fn change_tower_m_ed_knm_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().tower_m_ed_knm, Some(1562.5), "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: the diff must set tower_m_ed_knm to 1562.5");
    assert!(outcome.diff().artifact.is_none(), "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().tower_m_rd_knm.is_none(), "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: change-tower-m-ed-knm must leave tower_m_rd_knm untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-tower-m-ed-knm produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-tower-m-ed-knm committed diff decodes");
    assert_eq!(produced, committed, "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `towerMEdKnm` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-tower-m-ed-knm`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_tower_m_ed_knm_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-tower-m-ed-knm committed diff decodes");
    assert_eq!(decoded.tower_m_ed_knm, Some(1562.5), "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: the committed diff must carry tower_m_ed_knm at 1562.5");
    assert!(decoded.selected_check_index.is_none(), "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-tower-m-ed-knm committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-tower-m-ed-knm committed diff reparses");
    assert_eq!(reencoded, original, "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 1200.0 to 1562.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_tower_m_ed_knm_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-tower-m-ed-knm committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-tower-m-ed-knm committed diff applies to the before-snapshot");
    assert_eq!(produced.tower_m_ed_knm, 1562.5, "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: the committed diff must leave tower_m_ed_knm reading 1562.5");
    assert_eq!(produced, expected_after(), "change-tower-m-ed-knm/raises-tower-m-ed-knm-to-1562-5: the committed diff did not carry before to after");
}
