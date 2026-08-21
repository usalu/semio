//! 🧪️ `change-hd-over-h` fixture — `raises-hd-over-h-to-12-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1992Diff.hd_over_h` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.hd_over_h == payload.new_hd_over_h` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-hd-over-h before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-hd-over-h after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-hd-over-h mutation decodes")
}

/// ▶️ `change-hd-over-h` carries the committed before-snapshot to the committed after-snapshot by moving
/// `hd_over_h` from 10.0 to 12.5, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_hd_over_h_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-hd-over-h applies to its committed before-snapshot");
    assert_eq!(applied.hd_over_h, 12.5, "change-hd-over-h/raises-hd-over-h-to-12-5: hd_over_h must read 12.5 after the change");
    assert_eq!(applied, expected_after(), "change-hd-over-h/raises-hd-over-h-to-12-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-hd-over-h/raises-hd-over-h-to-12-5: a real 10.0 to 12.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-hd-over-h` is its own inverse partner: the inverse step restores `hd_over_h` to its pre-change
/// 10.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_hd_over_h_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-hd-over-h applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-hd-over-h/raises-hd-over-h-to-12-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-hd-over-h step applies");
        restored = next;
    }
    assert_eq!(restored.hd_over_h, 10.0, "change-hd-over-h/raises-hd-over-h-to-12-5: the inverse must put hd_over_h back to 10.0");
    assert_eq!(restored, base, "change-hd-over-h/raises-hd-over-h-to-12-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeHdOverH` payload are already canonical:
/// decode then encode is a fixed point, so `hdOverH` and `newHdOverH` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_hd_over_h_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-hd-over-h snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-hd-over-h snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-hd-over-h snapshot reparses");
        assert_eq!(reencoded, original, "change-hd-over-h/raises-hd-over-h-to-12-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-hd-over-h mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-hd-over-h mutation reparses");
    assert_eq!(reencoded, original, "change-hd-over-h/raises-hd-over-h-to-12-5: committed mutation JSON is not the canonical externally-tagged ChangeHdOverH form carrying newHdOverH");
}

/// 🎯️ The declared outcome holds: `change-hd-over-h` at 12.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_hd_over_h_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-hd-over-h outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-hd-over-h/raises-hd-over-h-to-12-5: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-hd-over-h/raises-hd-over-h-to-12-5: moving hd_over_h from 10.0 to 12.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-hd-over-h/raises-hd-over-h-to-12-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-hd-over-h` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `hdOverH` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `liquidSigmaSMpa`.
#[semio_framework_async_macros::async_test]
async fn change_hd_over_h_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().hd_over_h, Some(12.5), "change-hd-over-h/raises-hd-over-h-to-12-5: the diff must set hd_over_h to 12.5");
    assert!(outcome.diff().artifact.is_none(), "change-hd-over-h/raises-hd-over-h-to-12-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().liquid_sigma_s_mpa.is_none(), "change-hd-over-h/raises-hd-over-h-to-12-5: change-hd-over-h must leave liquid_sigma_s_mpa untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-hd-over-h produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-hd-over-h committed diff decodes");
    assert_eq!(produced, committed, "change-hd-over-h/raises-hd-over-h-to-12-5: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `hdOverH` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-hd-over-h`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_hd_over_h_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-hd-over-h committed diff decodes");
    assert_eq!(decoded.hd_over_h, Some(12.5), "change-hd-over-h/raises-hd-over-h-to-12-5: the committed diff must carry hd_over_h at 12.5");
    assert!(decoded.selected_check_index.is_none(), "change-hd-over-h/raises-hd-over-h-to-12-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-hd-over-h committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-hd-over-h committed diff reparses");
    assert_eq!(reencoded, original, "change-hd-over-h/raises-hd-over-h-to-12-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 10.0 to 12.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_hd_over_h_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-hd-over-h committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-hd-over-h committed diff applies to the before-snapshot");
    assert_eq!(produced.hd_over_h, 12.5, "change-hd-over-h/raises-hd-over-h-to-12-5: the committed diff must leave hd_over_h reading 12.5");
    assert_eq!(produced, expected_after(), "change-hd-over-h/raises-hd-over-h-to-12-5: the committed diff did not carry before to after");
}
