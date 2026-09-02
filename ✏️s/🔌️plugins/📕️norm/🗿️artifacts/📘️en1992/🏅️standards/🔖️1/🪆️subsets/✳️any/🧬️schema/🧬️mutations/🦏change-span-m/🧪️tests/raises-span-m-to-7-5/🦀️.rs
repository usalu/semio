//! 🧪️ `change-span-m` fixture — `raises-span-m-to-7-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1992Diff.span_m` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.span_m == payload.new_span_m` `mutation.no-op` guard.
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from these files by `fixtures generate` and asserted by the codec matrix, never hand-forged here.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1992Snapshot {
    serde_json::from_str(BEFORE).expect("change-span-m before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-span-m after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-span-m mutation decodes")
}

/// ▶️ `change-span-m` carries the committed before-snapshot to the committed after-snapshot by moving
/// `span_m` from 6.0 to 7.5, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
fn change_span_m_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-span-m applies to its committed before-snapshot");
    assert_eq!(applied.span_m, 7.5, "change-span-m/raises-span-m-to-7-5: span_m must read 7.5 after the change");
    assert_eq!(applied, expected_after(), "change-span-m/raises-span-m-to-7-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-span-m/raises-span-m-to-7-5: a real 6.0 to 7.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-span-m` is its own inverse partner: the inverse step restores `span_m` to its pre-change
/// 6.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_span_m_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-span-m applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-span-m/raises-span-m-to-7-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-span-m step applies");
        restored = next;
    }
    assert_eq!(restored.span_m, 6.0, "change-span-m/raises-span-m-to-7-5: the inverse must put span_m back to 6.0");
    assert_eq!(restored, base, "change-span-m/raises-span-m-to-7-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeSpanM` payload are already canonical:
/// decode then encode is a fixed point, so `spanM` and `newSpanM` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_span_m_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-span-m snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-span-m snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-span-m snapshot reparses");
        assert_eq!(reencoded, original, "change-span-m/raises-span-m-to-7-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-span-m mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-span-m mutation reparses");
    assert_eq!(reencoded, original, "change-span-m/raises-span-m-to-7-5: committed mutation JSON is not the canonical externally-tagged ChangeSpanM form carrying newSpanM");
}

/// 🎯️ The declared outcome holds: `change-span-m` at 7.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_span_m_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-span-m outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-span-m/raises-span-m-to-7-5: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-span-m/raises-span-m-to-7-5: moving span_m from 6.0 to 7.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-span-m/raises-span-m-to-7-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-span-m` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `spanM` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `udlKnM`.
#[semio_framework_async_macros::async_test]
fn change_span_m_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().span_m, Some(7.5), "change-span-m/raises-span-m-to-7-5: the diff must set span_m to 7.5");
    assert!(outcome.diff().artifact.is_none(), "change-span-m/raises-span-m-to-7-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().udl_kn_m.is_none(), "change-span-m/raises-span-m-to-7-5: change-span-m must leave udl_kn_m untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-span-m produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-span-m committed diff decodes");
    assert_eq!(produced, committed, "change-span-m/raises-span-m-to-7-5: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `spanM` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-span-m`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_span_m_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-span-m committed diff decodes");
    assert_eq!(decoded.span_m, Some(7.5), "change-span-m/raises-span-m-to-7-5: the committed diff must carry span_m at 7.5");
    assert!(decoded.selected_check_index.is_none(), "change-span-m/raises-span-m-to-7-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-span-m committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-span-m committed diff reparses");
    assert_eq!(reencoded, original, "change-span-m/raises-span-m-to-7-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 6.0 to 7.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_span_m_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-span-m committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-span-m committed diff applies to the before-snapshot");
    assert_eq!(produced.span_m, 7.5, "change-span-m/raises-span-m-to-7-5: the committed diff must leave span_m reading 7.5");
    assert_eq!(produced, expected_after(), "change-span-m/raises-span-m-to-7-5: the committed diff did not carry before to after");
}
