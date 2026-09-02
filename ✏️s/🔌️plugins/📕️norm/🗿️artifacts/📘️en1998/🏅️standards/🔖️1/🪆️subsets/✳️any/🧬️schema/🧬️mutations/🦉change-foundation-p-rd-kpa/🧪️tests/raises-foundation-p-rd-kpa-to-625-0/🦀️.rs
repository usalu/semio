//! 🧪️ `change-foundation-p-rd-kpa` fixture — `raises-foundation-p-rd-kpa-to-625-0`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.foundation_p_rd_kpa` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.foundation_p_rd_kpa == payload.new_foundation_p_rd_kpa` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-foundation-p-rd-kpa before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-foundation-p-rd-kpa after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-foundation-p-rd-kpa mutation decodes")
}

/// ▶️ `change-foundation-p-rd-kpa` carries the committed before-snapshot to the committed after-snapshot by moving
/// `foundation_p_rd_kpa` from 500.0 to 625.0, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_foundation_p_rd_kpa_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-foundation-p-rd-kpa applies to its committed before-snapshot");
    assert_eq!(applied.foundation_p_rd_kpa, 625.0, "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: foundation_p_rd_kpa must read 625.0 after the change");
    assert_eq!(applied, expected_after(), "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: a real 500.0 to 625.0 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-foundation-p-rd-kpa` is its own inverse partner: the inverse step restores `foundation_p_rd_kpa` to its pre-change
/// 500.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_foundation_p_rd_kpa_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-foundation-p-rd-kpa applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-foundation-p-rd-kpa step applies");
        restored = next;
    }
    assert_eq!(restored.foundation_p_rd_kpa, 500.0, "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: the inverse must put foundation_p_rd_kpa back to 500.0");
    assert_eq!(restored, base, "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeFoundationPRdKpa` payload are already canonical:
/// decode then encode is a fixed point, so `foundationPRdKpa` and `newFoundationPRdKpa` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_foundation_p_rd_kpa_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-foundation-p-rd-kpa snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-foundation-p-rd-kpa snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-foundation-p-rd-kpa snapshot reparses");
        assert_eq!(reencoded, original, "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-foundation-p-rd-kpa mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-foundation-p-rd-kpa mutation reparses");
    assert_eq!(reencoded, original, "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: committed mutation JSON is not the canonical externally-tagged ChangeFoundationPRdKpa form carrying newFoundationPRdKpa");
}

/// 🎯️ The declared outcome holds: `change-foundation-p-rd-kpa` at 625.0 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_foundation_p_rd_kpa_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-foundation-p-rd-kpa outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: moving foundation_p_rd_kpa from 500.0 to 625.0 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-foundation-p-rd-kpa` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `foundationPRdKpa` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `foundationHEdKn`.
#[semio_framework_async_macros::async_test]
fn change_foundation_p_rd_kpa_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().foundation_p_rd_kpa, Some(625.0), "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: the diff must set foundation_p_rd_kpa to 625.0");
    assert!(outcome.diff().artifact.is_none(), "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().foundation_h_ed_kn.is_none(), "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: change-foundation-p-rd-kpa must leave foundation_h_ed_kn untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-foundation-p-rd-kpa produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-foundation-p-rd-kpa committed diff decodes");
    assert_eq!(produced, committed, "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `foundationPRdKpa` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-foundation-p-rd-kpa`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_foundation_p_rd_kpa_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-foundation-p-rd-kpa committed diff decodes");
    assert_eq!(decoded.foundation_p_rd_kpa, Some(625.0), "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: the committed diff must carry foundation_p_rd_kpa at 625.0");
    assert!(decoded.selected_check_index.is_none(), "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-foundation-p-rd-kpa committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-foundation-p-rd-kpa committed diff reparses");
    assert_eq!(reencoded, original, "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 500.0 to 625.0 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_foundation_p_rd_kpa_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-foundation-p-rd-kpa committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-foundation-p-rd-kpa committed diff applies to the before-snapshot");
    assert_eq!(produced.foundation_p_rd_kpa, 625.0, "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: the committed diff must leave foundation_p_rd_kpa reading 625.0");
    assert_eq!(produced, expected_after(), "change-foundation-p-rd-kpa/raises-foundation-p-rd-kpa-to-625-0: the committed diff did not carry before to after");
}
