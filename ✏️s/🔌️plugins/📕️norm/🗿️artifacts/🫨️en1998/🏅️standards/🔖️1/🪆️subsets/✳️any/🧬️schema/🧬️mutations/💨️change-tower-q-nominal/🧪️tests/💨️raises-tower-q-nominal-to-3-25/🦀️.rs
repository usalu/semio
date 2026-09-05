//! 🧪️ `change-tower-q-nominal` fixture — `💨️raises-tower-q-nominal-to-3-25`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.tower_q_nominal` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.tower_q_nominal == payload.new_tower_q_nominal` `mutation.no-op` guard.
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from these files by `fixtures generate` and asserted by the codec matrix, never hand-forged here.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1998Snapshot {
    serde_json::from_str(BEFORE).expect("change-tower-q-nominal before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-tower-q-nominal after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-tower-q-nominal mutation decodes")
}

/// ▶️ `change-tower-q-nominal` carries the committed before-snapshot to the committed after-snapshot by moving
/// `tower_q_nominal` from 2.5 to 3.25, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_tower_q_nominal_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-tower-q-nominal applies to its committed before-snapshot");
    assert_eq!(applied.tower_q_nominal, 3.25, "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: tower_q_nominal must read 3.25 after the change");
    assert_eq!(applied, expected_after(), "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: a real 2.5 to 3.25 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-tower-q-nominal` is its own inverse partner: the inverse step restores `tower_q_nominal` to its pre-change
/// 2.5 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_tower_q_nominal_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-tower-q-nominal applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-tower-q-nominal step applies");
        restored = next;
    }
    assert_eq!(restored.tower_q_nominal, 2.5, "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: the inverse must put tower_q_nominal back to 2.5");
    assert_eq!(restored, base, "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeTowerQNominal` payload are already canonical:
/// decode then encode is a fixed point, so `towerQNominal` and `newTowerQNominal` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_tower_q_nominal_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-tower-q-nominal snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-tower-q-nominal snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-tower-q-nominal snapshot reparses");
        assert_eq!(reencoded, original, "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-tower-q-nominal mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-tower-q-nominal mutation reparses");
    assert_eq!(reencoded, original, "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: committed mutation JSON is not the canonical externally-tagged ChangeTowerQNominal form carrying newTowerQNominal");
}

/// 🎯️ The declared outcome holds: `change-tower-q-nominal` at 3.25 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_tower_q_nominal_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-tower-q-nominal outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: moving tower_q_nominal from 2.5 to 3.25 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-tower-q-nominal` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `towerQNominal` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `towerMassT`.
#[semio_framework_async_macros::async_test]
fn change_tower_q_nominal_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().tower_q_nominal, Some(3.25), "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: the diff must set tower_q_nominal to 3.25");
    assert!(outcome.diff().artifact.is_none(), "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().tower_mass_t.is_none(), "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: change-tower-q-nominal must leave tower_mass_t untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-tower-q-nominal produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-tower-q-nominal committed diff decodes");
    assert_eq!(produced, committed, "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `towerQNominal` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-tower-q-nominal`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_tower_q_nominal_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-tower-q-nominal committed diff decodes");
    assert_eq!(decoded.tower_q_nominal, Some(3.25), "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: the committed diff must carry tower_q_nominal at 3.25");
    assert!(decoded.selected_check_index.is_none(), "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-tower-q-nominal committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-tower-q-nominal committed diff reparses");
    assert_eq!(reencoded, original, "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 2.5 to 3.25 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_tower_q_nominal_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-tower-q-nominal committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-tower-q-nominal committed diff applies to the before-snapshot");
    assert_eq!(produced.tower_q_nominal, 3.25, "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: the committed diff must leave tower_q_nominal reading 3.25");
    assert_eq!(produced, expected_after(), "change-tower-q-nominal/raises-tower-q-nominal-to-3-25: the committed diff did not carry before to after");
}
