//! 🧪️ `change-en-a-gr` fixture — `raises-en-a-gr-to-0-25`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.en_a_gr` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.en_a_gr == payload.new_en_a_gr` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-en-a-gr before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-en-a-gr after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-en-a-gr mutation decodes")
}

/// ▶️ `change-en-a-gr` carries the committed before-snapshot to the committed after-snapshot by moving
/// `en_a_gr` from 0.15 to 0.25, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_en_a_gr_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-en-a-gr applies to its committed before-snapshot");
    assert_eq!(applied.en_a_gr, 0.25, "change-en-a-gr/raises-en-a-gr-to-0-25: en_a_gr must read 0.25 after the change");
    assert_eq!(applied, expected_after(), "change-en-a-gr/raises-en-a-gr-to-0-25: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-en-a-gr/raises-en-a-gr-to-0-25: a real 0.15 to 0.25 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-en-a-gr` is its own inverse partner: the inverse step restores `en_a_gr` to its pre-change
/// 0.15 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_en_a_gr_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-en-a-gr applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-en-a-gr/raises-en-a-gr-to-0-25: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-en-a-gr step applies");
        restored = next;
    }
    assert_eq!(restored.en_a_gr, 0.15, "change-en-a-gr/raises-en-a-gr-to-0-25: the inverse must put en_a_gr back to 0.15");
    assert_eq!(restored, base, "change-en-a-gr/raises-en-a-gr-to-0-25: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeEnAGr` payload are already canonical:
/// decode then encode is a fixed point, so `enAGr` and `newEnAGr` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_en_a_gr_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-en-a-gr snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-en-a-gr snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-en-a-gr snapshot reparses");
        assert_eq!(reencoded, original, "change-en-a-gr/raises-en-a-gr-to-0-25: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-en-a-gr mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-en-a-gr mutation reparses");
    assert_eq!(reencoded, original, "change-en-a-gr/raises-en-a-gr-to-0-25: committed mutation JSON is not the canonical externally-tagged ChangeEnAGr form carrying newEnAGr");
}

/// 🎯️ The declared outcome holds: `change-en-a-gr` at 0.25 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_en_a_gr_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-en-a-gr outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-en-a-gr/raises-en-a-gr-to-0-25: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-en-a-gr/raises-en-a-gr-to-0-25: moving en_a_gr from 0.15 to 0.25 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-en-a-gr/raises-en-a-gr-to-0-25: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-en-a-gr` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `enAGr` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `enGroundType`.
#[semio_framework_async_macros::async_test]
async fn change_en_a_gr_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().en_a_gr, Some(0.25), "change-en-a-gr/raises-en-a-gr-to-0-25: the diff must set en_a_gr to 0.25");
    assert!(outcome.diff().artifact.is_none(), "change-en-a-gr/raises-en-a-gr-to-0-25: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().en_ground_type.is_none(), "change-en-a-gr/raises-en-a-gr-to-0-25: change-en-a-gr must leave en_ground_type untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-en-a-gr produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-en-a-gr committed diff decodes");
    assert_eq!(produced, committed, "change-en-a-gr/raises-en-a-gr-to-0-25: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `enAGr` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-en-a-gr`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_en_a_gr_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-en-a-gr committed diff decodes");
    assert_eq!(decoded.en_a_gr, Some(0.25), "change-en-a-gr/raises-en-a-gr-to-0-25: the committed diff must carry en_a_gr at 0.25");
    assert!(decoded.selected_check_index.is_none(), "change-en-a-gr/raises-en-a-gr-to-0-25: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-en-a-gr committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-en-a-gr committed diff reparses");
    assert_eq!(reencoded, original, "change-en-a-gr/raises-en-a-gr-to-0-25: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 0.15 to 0.25 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_en_a_gr_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-en-a-gr committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-en-a-gr committed diff applies to the before-snapshot");
    assert_eq!(produced.en_a_gr, 0.25, "change-en-a-gr/raises-en-a-gr-to-0-25: the committed diff must leave en_a_gr reading 0.25");
    assert_eq!(produced, expected_after(), "change-en-a-gr/raises-en-a-gr-to-0-25: the committed diff did not carry before to after");
}
