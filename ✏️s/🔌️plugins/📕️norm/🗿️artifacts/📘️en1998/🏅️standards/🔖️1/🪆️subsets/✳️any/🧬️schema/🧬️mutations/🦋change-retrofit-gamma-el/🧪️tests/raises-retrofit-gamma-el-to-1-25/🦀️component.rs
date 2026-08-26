//! 🧪️ `change-retrofit-gamma-el` fixture — `raises-retrofit-gamma-el-to-1-25`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.retrofit_gamma_el` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.retrofit_gamma_el == payload.new_retrofit_gamma_el` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-retrofit-gamma-el before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-retrofit-gamma-el after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-retrofit-gamma-el mutation decodes")
}

/// ▶️ `change-retrofit-gamma-el` carries the committed before-snapshot to the committed after-snapshot by moving
/// `retrofit_gamma_el` from 1.0 to 1.25, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_retrofit_gamma_el_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-retrofit-gamma-el applies to its committed before-snapshot");
    assert_eq!(applied.retrofit_gamma_el, 1.25, "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: retrofit_gamma_el must read 1.25 after the change");
    assert_eq!(applied, expected_after(), "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: a real 1.0 to 1.25 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-retrofit-gamma-el` is its own inverse partner: the inverse step restores `retrofit_gamma_el` to its pre-change
/// 1.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_retrofit_gamma_el_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-retrofit-gamma-el applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-retrofit-gamma-el step applies");
        restored = next;
    }
    assert_eq!(restored.retrofit_gamma_el, 1.0, "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: the inverse must put retrofit_gamma_el back to 1.0");
    assert_eq!(restored, base, "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeRetrofitGammaEl` payload are already canonical:
/// decode then encode is a fixed point, so `retrofitGammaEl` and `newRetrofitGammaEl` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_retrofit_gamma_el_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-retrofit-gamma-el snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-retrofit-gamma-el snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-retrofit-gamma-el snapshot reparses");
        assert_eq!(reencoded, original, "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-retrofit-gamma-el mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-retrofit-gamma-el mutation reparses");
    assert_eq!(reencoded, original, "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: committed mutation JSON is not the canonical externally-tagged ChangeRetrofitGammaEl form carrying newRetrofitGammaEl");
}

/// 🎯️ The declared outcome holds: `change-retrofit-gamma-el` at 1.25 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_retrofit_gamma_el_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-retrofit-gamma-el outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: moving retrofit_gamma_el from 1.0 to 1.25 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-retrofit-gamma-el` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `retrofitGammaEl` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `siloHeightM`.
#[semio_framework_async_macros::async_test]
fn change_retrofit_gamma_el_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().retrofit_gamma_el, Some(1.25), "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: the diff must set retrofit_gamma_el to 1.25");
    assert!(outcome.diff().artifact.is_none(), "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().silo_height_m.is_none(), "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: change-retrofit-gamma-el must leave silo_height_m untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-retrofit-gamma-el produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-retrofit-gamma-el committed diff decodes");
    assert_eq!(produced, committed, "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `retrofitGammaEl` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-retrofit-gamma-el`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_retrofit_gamma_el_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-retrofit-gamma-el committed diff decodes");
    assert_eq!(decoded.retrofit_gamma_el, Some(1.25), "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: the committed diff must carry retrofit_gamma_el at 1.25");
    assert!(decoded.selected_check_index.is_none(), "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-retrofit-gamma-el committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-retrofit-gamma-el committed diff reparses");
    assert_eq!(reencoded, original, "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 1.0 to 1.25 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_retrofit_gamma_el_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-retrofit-gamma-el committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-retrofit-gamma-el committed diff applies to the before-snapshot");
    assert_eq!(produced.retrofit_gamma_el, 1.25, "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: the committed diff must leave retrofit_gamma_el reading 1.25");
    assert_eq!(produced, expected_after(), "change-retrofit-gamma-el/raises-retrofit-gamma-el-to-1-25: the committed diff did not carry before to after");
}
