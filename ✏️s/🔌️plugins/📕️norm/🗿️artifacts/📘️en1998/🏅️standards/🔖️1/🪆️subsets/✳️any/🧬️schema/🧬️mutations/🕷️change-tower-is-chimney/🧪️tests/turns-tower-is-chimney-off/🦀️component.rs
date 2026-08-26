//! 🧪️ `change-tower-is-chimney` fixture — `turns-tower-is-chimney-off`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.tower_is_chimney` and nothing else,
//! behind a `base.tower_is_chimney == payload.new_tower_is_chimney` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-tower-is-chimney before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-tower-is-chimney after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-tower-is-chimney mutation decodes")
}

/// ▶️ `change-tower-is-chimney` carries the committed before-snapshot to the committed after-snapshot by moving
/// `tower_is_chimney` from true to false, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_tower_is_chimney_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-tower-is-chimney applies to its committed before-snapshot");
    assert!(!applied.tower_is_chimney, "change-tower-is-chimney/turns-tower-is-chimney-off: tower_is_chimney must read false after the change");
    assert_eq!(applied, expected_after(), "change-tower-is-chimney/turns-tower-is-chimney-off: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-tower-is-chimney/turns-tower-is-chimney-off: a real true to false change must raise no `mutation.no-op` message");
}

/// ↩️ `change-tower-is-chimney` is its own inverse partner: the inverse step restores `tower_is_chimney` to its pre-change
/// true and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_tower_is_chimney_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-tower-is-chimney applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-tower-is-chimney/turns-tower-is-chimney-off: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-tower-is-chimney step applies");
        restored = next;
    }
    assert!(restored.tower_is_chimney, "change-tower-is-chimney/turns-tower-is-chimney-off: the inverse must put tower_is_chimney back to true");
    assert_eq!(restored, base, "change-tower-is-chimney/turns-tower-is-chimney-off: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeTowerIsChimney` payload are already canonical:
/// decode then encode is a fixed point, so `towerIsChimney` and `newTowerIsChimney` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_tower_is_chimney_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-tower-is-chimney snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-tower-is-chimney snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-tower-is-chimney snapshot reparses");
        assert_eq!(reencoded, original, "change-tower-is-chimney/turns-tower-is-chimney-off: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-tower-is-chimney mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-tower-is-chimney mutation reparses");
    assert_eq!(reencoded, original, "change-tower-is-chimney/turns-tower-is-chimney-off: committed mutation JSON is not the canonical externally-tagged ChangeTowerIsChimney form carrying newTowerIsChimney");
}

/// 🎯️ The declared outcome holds: `change-tower-is-chimney` at false is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_tower_is_chimney_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-tower-is-chimney outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-tower-is-chimney/turns-tower-is-chimney-off: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-tower-is-chimney/turns-tower-is-chimney-off: moving tower_is_chimney from true to false must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-tower-is-chimney/turns-tower-is-chimney-off: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-tower-is-chimney` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `towerIsChimney` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `towerQNominal`.
#[semio_framework_async_macros::async_test]
fn change_tower_is_chimney_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().tower_is_chimney, Some(false), "change-tower-is-chimney/turns-tower-is-chimney-off: the diff must set tower_is_chimney to false");
    assert!(outcome.diff().artifact.is_none(), "change-tower-is-chimney/turns-tower-is-chimney-off: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().tower_q_nominal.is_none(), "change-tower-is-chimney/turns-tower-is-chimney-off: change-tower-is-chimney must leave tower_q_nominal untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-tower-is-chimney produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-tower-is-chimney committed diff decodes");
    assert_eq!(produced, committed, "change-tower-is-chimney/turns-tower-is-chimney-off: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `towerIsChimney` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-tower-is-chimney`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_tower_is_chimney_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-tower-is-chimney committed diff decodes");
    assert_eq!(decoded.tower_is_chimney, Some(false), "change-tower-is-chimney/turns-tower-is-chimney-off: the committed diff must carry tower_is_chimney at false");
    assert!(decoded.selected_check_index.is_none(), "change-tower-is-chimney/turns-tower-is-chimney-off: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-tower-is-chimney committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-tower-is-chimney committed diff reparses");
    assert_eq!(reencoded, original, "change-tower-is-chimney/turns-tower-is-chimney-off: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the true to false delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_tower_is_chimney_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-tower-is-chimney committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-tower-is-chimney committed diff applies to the before-snapshot");
    assert!(!produced.tower_is_chimney, "change-tower-is-chimney/turns-tower-is-chimney-off: the committed diff must leave tower_is_chimney reading false");
    assert_eq!(produced, expected_after(), "change-tower-is-chimney/turns-tower-is-chimney-off: the committed diff did not carry before to after");
}
