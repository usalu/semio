//! 🧪️ `change-retrofit-ed-kn` fixture — `raises-retrofit-e-d-kn-to-337-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.retrofit_e_d_kn` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.retrofit_e_d_kn == payload.new_retrofit_e_d_kn` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-retrofit-ed-kn before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-retrofit-ed-kn after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-retrofit-ed-kn mutation decodes")
}

/// ▶️ `change-retrofit-ed-kn` carries the committed before-snapshot to the committed after-snapshot by moving
/// `retrofit_e_d_kn` from 250.0 to 337.5, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_retrofit_ed_kn_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-retrofit-ed-kn applies to its committed before-snapshot");
    assert_eq!(applied.retrofit_e_d_kn, 337.5, "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: retrofit_e_d_kn must read 337.5 after the change");
    assert_eq!(applied, expected_after(), "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: a real 250.0 to 337.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-retrofit-ed-kn` is its own inverse partner: the inverse step restores `retrofit_e_d_kn` to its pre-change
/// 250.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_retrofit_ed_kn_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-retrofit-ed-kn applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-retrofit-ed-kn step applies");
        restored = next;
    }
    assert_eq!(restored.retrofit_e_d_kn, 250.0, "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: the inverse must put retrofit_e_d_kn back to 250.0");
    assert_eq!(restored, base, "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeRetrofitEDKn` payload are already canonical:
/// decode then encode is a fixed point, so `retrofitEDKn` and `newRetrofitEDKn` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_retrofit_ed_kn_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-retrofit-ed-kn snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-retrofit-ed-kn snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-retrofit-ed-kn snapshot reparses");
        assert_eq!(reencoded, original, "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-retrofit-ed-kn mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-retrofit-ed-kn mutation reparses");
    assert_eq!(reencoded, original, "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: committed mutation JSON is not the canonical externally-tagged ChangeRetrofitEDKn form carrying newRetrofitEDKn");
}

/// 🎯️ The declared outcome holds: `change-retrofit-ed-kn` at 337.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_retrofit_ed_kn_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-retrofit-ed-kn outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: moving retrofit_e_d_kn from 250.0 to 337.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-retrofit-ed-kn` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `retrofitEDKn` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `retrofitRKKn`.
#[semio_framework_async_macros::async_test]
fn change_retrofit_ed_kn_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().retrofit_e_d_kn, Some(337.5), "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: the diff must set retrofit_e_d_kn to 337.5");
    assert!(outcome.diff().artifact.is_none(), "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().retrofit_r_k_kn.is_none(), "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: change-retrofit-ed-kn must leave retrofit_r_k_kn untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-retrofit-ed-kn produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-retrofit-ed-kn committed diff decodes");
    assert_eq!(produced, committed, "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `retrofitEDKn` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-retrofit-ed-kn`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_retrofit_ed_kn_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-retrofit-ed-kn committed diff decodes");
    assert_eq!(decoded.retrofit_e_d_kn, Some(337.5), "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: the committed diff must carry retrofit_e_d_kn at 337.5");
    assert!(decoded.selected_check_index.is_none(), "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-retrofit-ed-kn committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-retrofit-ed-kn committed diff reparses");
    assert_eq!(reencoded, original, "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 250.0 to 337.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_retrofit_ed_kn_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-retrofit-ed-kn committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-retrofit-ed-kn committed diff applies to the before-snapshot");
    assert_eq!(produced.retrofit_e_d_kn, 337.5, "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: the committed diff must leave retrofit_e_d_kn reading 337.5");
    assert_eq!(produced, expected_after(), "change-retrofit-ed-kn/raises-retrofit-e-d-kn-to-337-5: the committed diff did not carry before to after");
}
