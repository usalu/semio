//! 🧪️ `change-retrofit-limit-state` fixture — `switches-retrofit-limit-state-to-near-collapse`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.retrofit_limit_state` and nothing else,
//! behind a `base.retrofit_limit_state == payload.new_retrofit_limit_state` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-retrofit-limit-state before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-retrofit-limit-state after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-retrofit-limit-state mutation decodes")
}

/// ▶️ `change-retrofit-limit-state` carries the committed before-snapshot to the committed after-snapshot by moving
/// `retrofit_limit_state` from significant_damage to near_collapse, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_retrofit_limit_state_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-retrofit-limit-state applies to its committed before-snapshot");
    assert_eq!(applied.retrofit_limit_state, "near_collapse", "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: retrofit_limit_state must read near_collapse after the change");
    assert_eq!(applied, expected_after(), "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: a real significant_damage to near_collapse change must raise no `mutation.no-op` message");
}

/// ↩️ `change-retrofit-limit-state` is its own inverse partner: the inverse step restores `retrofit_limit_state` to its pre-change
/// significant_damage and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_retrofit_limit_state_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-retrofit-limit-state applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-retrofit-limit-state step applies");
        restored = next;
    }
    assert_eq!(restored.retrofit_limit_state, "significant_damage", "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: the inverse must put retrofit_limit_state back to significant_damage");
    assert_eq!(restored, base, "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeRetrofitLimitState` payload are already canonical:
/// decode then encode is a fixed point, so `retrofitLimitState` and `newRetrofitLimitState` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_retrofit_limit_state_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-retrofit-limit-state snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-retrofit-limit-state snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-retrofit-limit-state snapshot reparses");
        assert_eq!(reencoded, original, "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-retrofit-limit-state mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-retrofit-limit-state mutation reparses");
    assert_eq!(reencoded, original, "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: committed mutation JSON is not the canonical externally-tagged ChangeRetrofitLimitState form carrying newRetrofitLimitState");
}

/// 🎯️ The declared outcome holds: `change-retrofit-limit-state` at near_collapse is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_retrofit_limit_state_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-retrofit-limit-state outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: moving retrofit_limit_state from significant_damage to near_collapse must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-retrofit-limit-state` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `retrofitLimitState` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `retrofitEDKn`.
#[semio_framework_async_macros::async_test]
fn change_retrofit_limit_state_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().retrofit_limit_state.as_deref(), Some("near_collapse"), "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: the diff must set retrofit_limit_state to near_collapse");
    assert!(outcome.diff().artifact.is_none(), "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().retrofit_e_d_kn.is_none(), "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: change-retrofit-limit-state must leave retrofit_e_d_kn untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-retrofit-limit-state produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-retrofit-limit-state committed diff decodes");
    assert_eq!(produced, committed, "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `retrofitLimitState` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-retrofit-limit-state`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_retrofit_limit_state_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-retrofit-limit-state committed diff decodes");
    assert_eq!(decoded.retrofit_limit_state.as_deref(), Some("near_collapse"), "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: the committed diff must carry retrofit_limit_state at near_collapse");
    assert!(decoded.selected_check_index.is_none(), "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-retrofit-limit-state committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-retrofit-limit-state committed diff reparses");
    assert_eq!(reencoded, original, "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the significant_damage to near_collapse delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_retrofit_limit_state_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-retrofit-limit-state committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-retrofit-limit-state committed diff applies to the before-snapshot");
    assert_eq!(produced.retrofit_limit_state, "near_collapse", "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: the committed diff must leave retrofit_limit_state reading near_collapse");
    assert_eq!(produced, expected_after(), "change-retrofit-limit-state/switches-retrofit-limit-state-to-near-collapse: the committed diff did not carry before to after");
}
