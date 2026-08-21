//! 🧪️ `change-f-yk` fixture — `raises-f-yk-to-550-0`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1992Diff.f_yk` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.f_yk == payload.new_f_yk` `mutation.no-op` guard.
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from these files by `fixtures generate` and asserted by the codec matrix, never hand-forged here.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1992Snapshot {
    serde_json::from_str(BEFORE).expect("change-f-yk before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-f-yk after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-f-yk mutation decodes")
}

/// ▶️ `change-f-yk` carries the committed before-snapshot to the committed after-snapshot by moving
/// `f_yk` from 500.0 to 550.0, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_f_yk_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-f-yk applies to its committed before-snapshot");
    assert_eq!(applied.f_yk, 550.0, "change-f-yk/raises-f-yk-to-550-0: f_yk must read 550.0 after the change");
    assert_eq!(applied, expected_after(), "change-f-yk/raises-f-yk-to-550-0: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-f-yk/raises-f-yk-to-550-0: a real 500.0 to 550.0 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-f-yk` is its own inverse partner: the inverse step restores `f_yk` to its pre-change
/// 500.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_f_yk_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-f-yk applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-f-yk/raises-f-yk-to-550-0: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-f-yk step applies");
        restored = next;
    }
    assert_eq!(restored.f_yk, 500.0, "change-f-yk/raises-f-yk-to-550-0: the inverse must put f_yk back to 500.0");
    assert_eq!(restored, base, "change-f-yk/raises-f-yk-to-550-0: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeFYk` payload are already canonical:
/// decode then encode is a fixed point, so `fYk` and `newFYk` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_f_yk_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-f-yk snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-f-yk snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-f-yk snapshot reparses");
        assert_eq!(reencoded, original, "change-f-yk/raises-f-yk-to-550-0: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-f-yk mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-f-yk mutation reparses");
    assert_eq!(reencoded, original, "change-f-yk/raises-f-yk-to-550-0: committed mutation JSON is not the canonical externally-tagged ChangeFYk form carrying newFYk");
}

/// 🎯️ The declared outcome holds: `change-f-yk` at 550.0 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_f_yk_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-f-yk outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-f-yk/raises-f-yk-to-550-0: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-f-yk/raises-f-yk-to-550-0: moving f_yk from 500.0 to 550.0 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-f-yk/raises-f-yk-to-550-0: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-f-yk` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `fYk` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `rhoL`.
#[semio_framework_async_macros::async_test]
async fn change_f_yk_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().f_yk, Some(550.0), "change-f-yk/raises-f-yk-to-550-0: the diff must set f_yk to 550.0");
    assert!(outcome.diff().artifact.is_none(), "change-f-yk/raises-f-yk-to-550-0: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().rho_l.is_none(), "change-f-yk/raises-f-yk-to-550-0: change-f-yk must leave rho_l untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-f-yk produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-f-yk committed diff decodes");
    assert_eq!(produced, committed, "change-f-yk/raises-f-yk-to-550-0: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `fYk` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-f-yk`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_f_yk_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-f-yk committed diff decodes");
    assert_eq!(decoded.f_yk, Some(550.0), "change-f-yk/raises-f-yk-to-550-0: the committed diff must carry f_yk at 550.0");
    assert!(decoded.selected_check_index.is_none(), "change-f-yk/raises-f-yk-to-550-0: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-f-yk committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-f-yk committed diff reparses");
    assert_eq!(reencoded, original, "change-f-yk/raises-f-yk-to-550-0: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 500.0 to 550.0 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_f_yk_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-f-yk committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-f-yk committed diff applies to the before-snapshot");
    assert_eq!(produced.f_yk, 550.0, "change-f-yk/raises-f-yk-to-550-0: the committed diff must leave f_yk reading 550.0");
    assert_eq!(produced, expected_after(), "change-f-yk/raises-f-yk-to-550-0: the committed diff did not carry before to after");
}
