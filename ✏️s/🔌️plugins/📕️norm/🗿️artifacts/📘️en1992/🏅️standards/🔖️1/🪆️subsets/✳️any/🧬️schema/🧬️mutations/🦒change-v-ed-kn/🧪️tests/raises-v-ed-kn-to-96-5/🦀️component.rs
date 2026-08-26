//! 🧪️ `change-v-ed-kn` fixture — `raises-v-ed-kn-to-96-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1992Diff.v_ed_kn` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.v_ed_kn == payload.new_v_ed_kn` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-v-ed-kn before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-v-ed-kn after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-v-ed-kn mutation decodes")
}

/// ▶️ `change-v-ed-kn` carries the committed before-snapshot to the committed after-snapshot by moving
/// `v_ed_kn` from 80.0 to 96.5, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
fn change_v_ed_kn_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-v-ed-kn applies to its committed before-snapshot");
    assert_eq!(applied.v_ed_kn, 96.5, "change-v-ed-kn/raises-v-ed-kn-to-96-5: v_ed_kn must read 96.5 after the change");
    assert_eq!(applied, expected_after(), "change-v-ed-kn/raises-v-ed-kn-to-96-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-v-ed-kn/raises-v-ed-kn-to-96-5: a real 80.0 to 96.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-v-ed-kn` is its own inverse partner: the inverse step restores `v_ed_kn` to its pre-change
/// 80.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_v_ed_kn_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-v-ed-kn applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-v-ed-kn/raises-v-ed-kn-to-96-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-v-ed-kn step applies");
        restored = next;
    }
    assert_eq!(restored.v_ed_kn, 80.0, "change-v-ed-kn/raises-v-ed-kn-to-96-5: the inverse must put v_ed_kn back to 80.0");
    assert_eq!(restored, base, "change-v-ed-kn/raises-v-ed-kn-to-96-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeVEdKn` payload are already canonical:
/// decode then encode is a fixed point, so `vEdKn` and `newVEdKn` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_v_ed_kn_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-v-ed-kn snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-v-ed-kn snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-v-ed-kn snapshot reparses");
        assert_eq!(reencoded, original, "change-v-ed-kn/raises-v-ed-kn-to-96-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-v-ed-kn mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-v-ed-kn mutation reparses");
    assert_eq!(reencoded, original, "change-v-ed-kn/raises-v-ed-kn-to-96-5: committed mutation JSON is not the canonical externally-tagged ChangeVEdKn form carrying newVEdKn");
}

/// 🎯️ The declared outcome holds: `change-v-ed-kn` at 96.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_v_ed_kn_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-v-ed-kn outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-v-ed-kn/raises-v-ed-kn-to-96-5: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-v-ed-kn/raises-v-ed-kn-to-96-5: moving v_ed_kn from 80.0 to 96.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-v-ed-kn/raises-v-ed-kn-to-96-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-v-ed-kn` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `vEdKn` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `fCk`.
#[semio_framework_async_macros::async_test]
fn change_v_ed_kn_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().v_ed_kn, Some(96.5), "change-v-ed-kn/raises-v-ed-kn-to-96-5: the diff must set v_ed_kn to 96.5");
    assert!(outcome.diff().artifact.is_none(), "change-v-ed-kn/raises-v-ed-kn-to-96-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().f_ck.is_none(), "change-v-ed-kn/raises-v-ed-kn-to-96-5: change-v-ed-kn must leave f_ck untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-v-ed-kn produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-v-ed-kn committed diff decodes");
    assert_eq!(produced, committed, "change-v-ed-kn/raises-v-ed-kn-to-96-5: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `vEdKn` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-v-ed-kn`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_v_ed_kn_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-v-ed-kn committed diff decodes");
    assert_eq!(decoded.v_ed_kn, Some(96.5), "change-v-ed-kn/raises-v-ed-kn-to-96-5: the committed diff must carry v_ed_kn at 96.5");
    assert!(decoded.selected_check_index.is_none(), "change-v-ed-kn/raises-v-ed-kn-to-96-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-v-ed-kn committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-v-ed-kn committed diff reparses");
    assert_eq!(reencoded, original, "change-v-ed-kn/raises-v-ed-kn-to-96-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 80.0 to 96.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_v_ed_kn_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-v-ed-kn committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-v-ed-kn committed diff applies to the before-snapshot");
    assert_eq!(produced.v_ed_kn, 96.5, "change-v-ed-kn/raises-v-ed-kn-to-96-5: the committed diff must leave v_ed_kn reading 96.5");
    assert_eq!(produced, expected_after(), "change-v-ed-kn/raises-v-ed-kn-to-96-5: the committed diff did not carry before to after");
}
