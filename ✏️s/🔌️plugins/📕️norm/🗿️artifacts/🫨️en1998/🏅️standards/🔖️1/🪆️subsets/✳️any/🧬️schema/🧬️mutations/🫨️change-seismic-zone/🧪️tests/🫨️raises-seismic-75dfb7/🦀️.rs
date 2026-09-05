//! 🫨️ `change-seismic-zone` fixture — `🫨️raises-seismic-zone-to-4`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.seismic_zone` and nothing else,
//! behind a `base.seismic_zone == payload.new_seismic_zone` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-seismic-zone before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-seismic-zone after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-seismic-zone mutation decodes")
}

/// ▶️ `change-seismic-zone` carries the committed before-snapshot to the committed after-snapshot by moving
/// `seismic_zone` from 2 to 4, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_seismic_zone_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-seismic-zone applies to its committed before-snapshot");
    assert_eq!(applied.seismic_zone, 4, "change-seismic-zone/raises-seismic-zone-to-4: seismic_zone must read 4 after the change");
    assert_eq!(applied, expected_after(), "change-seismic-zone/raises-seismic-zone-to-4: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-seismic-zone/raises-seismic-zone-to-4: a real 2 to 4 change must raise no `mutation.no-op` message");
}

/// ↩️ `change-seismic-zone` is its own inverse partner: the inverse step restores `seismic_zone` to its pre-change
/// 2 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_seismic_zone_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-seismic-zone applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-seismic-zone/raises-seismic-zone-to-4: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-seismic-zone step applies");
        restored = next;
    }
    assert_eq!(restored.seismic_zone, 2, "change-seismic-zone/raises-seismic-zone-to-4: the inverse must put seismic_zone back to 2");
    assert_eq!(restored, base, "change-seismic-zone/raises-seismic-zone-to-4: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeSeismicZone` payload are already canonical:
/// decode then encode is a fixed point, so `seismicZone` and `newSeismicZone` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_seismic_zone_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-seismic-zone snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-seismic-zone snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-seismic-zone snapshot reparses");
        assert_eq!(reencoded, original, "change-seismic-zone/raises-seismic-zone-to-4: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-seismic-zone mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-seismic-zone mutation reparses");
    assert_eq!(reencoded, original, "change-seismic-zone/raises-seismic-zone-to-4: committed mutation JSON is not the canonical externally-tagged ChangeSeismicZone form carrying newSeismicZone");
}

/// 🎯️ The declared outcome holds: `change-seismic-zone` at 4 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_seismic_zone_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-seismic-zone outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-seismic-zone/raises-seismic-zone-to-4: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-seismic-zone/raises-seismic-zone-to-4: moving seismic_zone from 2 to 4 must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-seismic-zone/raises-seismic-zone-to-4: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-seismic-zone` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `seismicZone` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `groundType`.
#[semio_framework_async_macros::async_test]
fn change_seismic_zone_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().seismic_zone, Some(4), "change-seismic-zone/raises-seismic-zone-to-4: the diff must set seismic_zone to 4");
    assert!(outcome.diff().artifact.is_none(), "change-seismic-zone/raises-seismic-zone-to-4: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().ground_type.is_none(), "change-seismic-zone/raises-seismic-zone-to-4: change-seismic-zone must leave ground_type untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-seismic-zone produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-seismic-zone committed diff decodes");
    assert_eq!(produced, committed, "change-seismic-zone/raises-seismic-zone-to-4: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `seismicZone` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-seismic-zone`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_seismic_zone_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-seismic-zone committed diff decodes");
    assert_eq!(decoded.seismic_zone, Some(4), "change-seismic-zone/raises-seismic-zone-to-4: the committed diff must carry seismic_zone at 4");
    assert!(decoded.selected_check_index.is_none(), "change-seismic-zone/raises-seismic-zone-to-4: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-seismic-zone committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-seismic-zone committed diff reparses");
    assert_eq!(reencoded, original, "change-seismic-zone/raises-seismic-zone-to-4: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 2 to 4 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_seismic_zone_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-seismic-zone committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-seismic-zone committed diff applies to the before-snapshot");
    assert_eq!(produced.seismic_zone, 4, "change-seismic-zone/raises-seismic-zone-to-4: the committed diff must leave seismic_zone reading 4");
    assert_eq!(produced, expected_after(), "change-seismic-zone/raises-seismic-zone-to-4: the committed diff did not carry before to after");
}
