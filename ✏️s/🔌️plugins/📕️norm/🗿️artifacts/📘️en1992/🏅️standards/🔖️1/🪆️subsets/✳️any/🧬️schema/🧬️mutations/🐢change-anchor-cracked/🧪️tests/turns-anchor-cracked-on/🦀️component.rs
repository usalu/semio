//! 🧪️ `change-anchor-cracked` fixture — `turns-anchor-cracked-on`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1992Diff.anchor_cracked` and nothing else,
//! behind a `base.anchor_cracked == payload.new_anchor_cracked` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-anchor-cracked before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-anchor-cracked after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-anchor-cracked mutation decodes")
}

/// ▶️ `change-anchor-cracked` carries the committed before-snapshot to the committed after-snapshot by moving
/// `anchor_cracked` from false to true, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_anchor_cracked_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-anchor-cracked applies to its committed before-snapshot");
    assert!(applied.anchor_cracked, "change-anchor-cracked/turns-anchor-cracked-on: anchor_cracked must read true after the change");
    assert_eq!(applied, expected_after(), "change-anchor-cracked/turns-anchor-cracked-on: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-anchor-cracked/turns-anchor-cracked-on: a real false to true change must raise no `mutation.no-op` message");
}

/// ↩️ `change-anchor-cracked` is its own inverse partner: the inverse step restores `anchor_cracked` to its pre-change
/// false and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_anchor_cracked_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-anchor-cracked applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-anchor-cracked/turns-anchor-cracked-on: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-anchor-cracked step applies");
        restored = next;
    }
    assert!(!restored.anchor_cracked, "change-anchor-cracked/turns-anchor-cracked-on: the inverse must put anchor_cracked back to false");
    assert_eq!(restored, base, "change-anchor-cracked/turns-anchor-cracked-on: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeAnchorCracked` payload are already canonical:
/// decode then encode is a fixed point, so `anchorCracked` and `newAnchorCracked` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_anchor_cracked_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-anchor-cracked snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-anchor-cracked snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-anchor-cracked snapshot reparses");
        assert_eq!(reencoded, original, "change-anchor-cracked/turns-anchor-cracked-on: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-anchor-cracked mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-anchor-cracked mutation reparses");
    assert_eq!(reencoded, original, "change-anchor-cracked/turns-anchor-cracked-on: committed mutation JSON is not the canonical externally-tagged ChangeAnchorCracked form carrying newAnchorCracked");
}

/// 🎯️ The declared outcome holds: `change-anchor-cracked` at true is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_anchor_cracked_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-anchor-cracked outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-anchor-cracked/turns-anchor-cracked-on: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-anchor-cracked/turns-anchor-cracked-on: moving anchor_cracked from false to true must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-anchor-cracked/turns-anchor-cracked-on: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-anchor-cracked` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `anchorCracked` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `anchorFUkMpa`.
#[semio_framework_async_macros::async_test]
async fn change_anchor_cracked_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().anchor_cracked, Some(true), "change-anchor-cracked/turns-anchor-cracked-on: the diff must set anchor_cracked to true");
    assert!(outcome.diff().artifact.is_none(), "change-anchor-cracked/turns-anchor-cracked-on: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().anchor_f_uk_mpa.is_none(), "change-anchor-cracked/turns-anchor-cracked-on: change-anchor-cracked must leave anchor_f_uk_mpa untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-anchor-cracked produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-anchor-cracked committed diff decodes");
    assert_eq!(produced, committed, "change-anchor-cracked/turns-anchor-cracked-on: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `anchorCracked` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-anchor-cracked`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_anchor_cracked_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-anchor-cracked committed diff decodes");
    assert_eq!(decoded.anchor_cracked, Some(true), "change-anchor-cracked/turns-anchor-cracked-on: the committed diff must carry anchor_cracked at true");
    assert!(decoded.selected_check_index.is_none(), "change-anchor-cracked/turns-anchor-cracked-on: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-anchor-cracked committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-anchor-cracked committed diff reparses");
    assert_eq!(reencoded, original, "change-anchor-cracked/turns-anchor-cracked-on: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the false to true delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_anchor_cracked_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-anchor-cracked committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-anchor-cracked committed diff applies to the before-snapshot");
    assert!(produced.anchor_cracked, "change-anchor-cracked/turns-anchor-cracked-on: the committed diff must leave anchor_cracked reading true");
    assert_eq!(produced, expected_after(), "change-anchor-cracked/turns-anchor-cracked-on: the committed diff did not carry before to after");
}
