//! 🧪️ `change-t1-s` fixture — `raises-t1-s-to-0-75`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.t1_s` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.t1_s == payload.new_t1_s` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-t1-s before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-t1-s after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-t1-s mutation decodes")
}

/// ▶️ `change-t1-s` carries the committed before-snapshot to the committed after-snapshot by moving
/// `t1_s` from 0.3 to 0.75, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_t1_s_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-t1-s applies to its committed before-snapshot");
    assert_eq!(applied.t1_s, 0.75, "change-t1-s/raises-t1-s-to-0-75: t1_s must read 0.75 after the change");
    assert_eq!(applied, expected_after(), "change-t1-s/raises-t1-s-to-0-75: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-t1-s/raises-t1-s-to-0-75: a real 0.3 to 0.75 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-t1-s` is its own inverse partner: the inverse step restores `t1_s` to its pre-change
/// 0.3 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_t1_s_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-t1-s applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-t1-s/raises-t1-s-to-0-75: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-t1-s step applies");
        restored = next;
    }
    assert_eq!(restored.t1_s, 0.3, "change-t1-s/raises-t1-s-to-0-75: the inverse must put t1_s back to 0.3");
    assert_eq!(restored, base, "change-t1-s/raises-t1-s-to-0-75: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeT1S` payload are already canonical:
/// decode then encode is a fixed point, so `t1S` and `newT1S` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_t1_s_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-t1-s snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-t1-s snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-t1-s snapshot reparses");
        assert_eq!(reencoded, original, "change-t1-s/raises-t1-s-to-0-75: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-t1-s mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-t1-s mutation reparses");
    assert_eq!(reencoded, original, "change-t1-s/raises-t1-s-to-0-75: committed mutation JSON is not the canonical externally-tagged ChangeT1S form carrying newT1S");
}

/// 🎯️ The declared outcome holds: `change-t1-s` at 0.75 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_t1_s_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-t1-s outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-t1-s/raises-t1-s-to-0-75: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-t1-s/raises-t1-s-to-0-75: moving t1_s from 0.3 to 0.75 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-t1-s/raises-t1-s-to-0-75: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-t1-s` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `t1S` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `massT`.
#[semio_framework_async_macros::async_test]
fn change_t1_s_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().t1_s, Some(0.75), "change-t1-s/raises-t1-s-to-0-75: the diff must set t1_s to 0.75");
    assert!(outcome.diff().artifact.is_none(), "change-t1-s/raises-t1-s-to-0-75: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().mass_t.is_none(), "change-t1-s/raises-t1-s-to-0-75: change-t1-s must leave mass_t untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-t1-s produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-t1-s committed diff decodes");
    assert_eq!(produced, committed, "change-t1-s/raises-t1-s-to-0-75: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `t1S` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-t1-s`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_t1_s_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-t1-s committed diff decodes");
    assert_eq!(decoded.t1_s, Some(0.75), "change-t1-s/raises-t1-s-to-0-75: the committed diff must carry t1_s at 0.75");
    assert!(decoded.selected_check_index.is_none(), "change-t1-s/raises-t1-s-to-0-75: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-t1-s committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-t1-s committed diff reparses");
    assert_eq!(reencoded, original, "change-t1-s/raises-t1-s-to-0-75: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 0.3 to 0.75 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_t1_s_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-t1-s committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-t1-s committed diff applies to the before-snapshot");
    assert_eq!(produced.t1_s, 0.75, "change-t1-s/raises-t1-s-to-0-75: the committed diff must leave t1_s reading 0.75");
    assert_eq!(produced, expected_after(), "change-t1-s/raises-t1-s-to-0-75: the committed diff did not carry before to after");
}
