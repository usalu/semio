//! 🧪️ `change-foundation-area-m2` fixture — `raises-foundation-area-m2-to-144-0`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.foundation_area_m2` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.foundation_area_m2 == payload.new_foundation_area_m2` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-foundation-area-m2 before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-foundation-area-m2 after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-foundation-area-m2 mutation decodes")
}

/// ▶️ `change-foundation-area-m2` carries the committed before-snapshot to the committed after-snapshot by moving
/// `foundation_area_m2` from 100.0 to 144.0, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_foundation_area_m2_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-foundation-area-m2 applies to its committed before-snapshot");
    assert_eq!(applied.foundation_area_m2, 144.0, "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: foundation_area_m2 must read 144.0 after the change");
    assert_eq!(applied, expected_after(), "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: a real 100.0 to 144.0 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-foundation-area-m2` is its own inverse partner: the inverse step restores `foundation_area_m2` to its pre-change
/// 100.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_foundation_area_m2_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-foundation-area-m2 applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-foundation-area-m2 step applies");
        restored = next;
    }
    assert_eq!(restored.foundation_area_m2, 100.0, "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: the inverse must put foundation_area_m2 back to 100.0");
    assert_eq!(restored, base, "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeFoundationAreaM2` payload are already canonical:
/// decode then encode is a fixed point, so `foundationAreaM2` and `newFoundationAreaM2` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_foundation_area_m2_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-foundation-area-m2 snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-foundation-area-m2 snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-foundation-area-m2 snapshot reparses");
        assert_eq!(reencoded, original, "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-foundation-area-m2 mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-foundation-area-m2 mutation reparses");
    assert_eq!(reencoded, original, "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: committed mutation JSON is not the canonical externally-tagged ChangeFoundationAreaM2 form carrying newFoundationAreaM2");
}

/// 🎯️ The declared outcome holds: `change-foundation-area-m2` at 144.0 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_foundation_area_m2_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-foundation-area-m2 outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: moving foundation_area_m2 from 100.0 to 144.0 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-foundation-area-m2` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `foundationAreaM2` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `foundationPRdKpa`.
#[semio_framework_async_macros::async_test]
fn change_foundation_area_m2_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().foundation_area_m2, Some(144.0), "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: the diff must set foundation_area_m2 to 144.0");
    assert!(outcome.diff().artifact.is_none(), "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().foundation_p_rd_kpa.is_none(), "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: change-foundation-area-m2 must leave foundation_p_rd_kpa untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-foundation-area-m2 produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-foundation-area-m2 committed diff decodes");
    assert_eq!(produced, committed, "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `foundationAreaM2` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-foundation-area-m2`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_foundation_area_m2_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-foundation-area-m2 committed diff decodes");
    assert_eq!(decoded.foundation_area_m2, Some(144.0), "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: the committed diff must carry foundation_area_m2 at 144.0");
    assert!(decoded.selected_check_index.is_none(), "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-foundation-area-m2 committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-foundation-area-m2 committed diff reparses");
    assert_eq!(reencoded, original, "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 100.0 to 144.0 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_foundation_area_m2_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-foundation-area-m2 committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-foundation-area-m2 committed diff applies to the before-snapshot");
    assert_eq!(produced.foundation_area_m2, 144.0, "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: the committed diff must leave foundation_area_m2 reading 144.0");
    assert_eq!(produced, expected_after(), "change-foundation-area-m2/raises-foundation-area-m2-to-144-0: the committed diff did not carry before to after");
}
