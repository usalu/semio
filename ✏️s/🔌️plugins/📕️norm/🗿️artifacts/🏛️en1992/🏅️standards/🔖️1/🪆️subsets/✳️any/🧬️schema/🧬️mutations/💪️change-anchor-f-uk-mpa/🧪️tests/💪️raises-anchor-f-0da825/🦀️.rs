//! 🧪️ `change-anchor-f-uk-mpa` fixture — `💪️raises-anchor-f-uk-mpa-to-900-0`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1992Diff.anchor_f_uk_mpa` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.anchor_f_uk_mpa == payload.new_anchor_f_uk_mpa` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-anchor-f-uk-mpa before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-anchor-f-uk-mpa after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-anchor-f-uk-mpa mutation decodes")
}

/// ▶️ `change-anchor-f-uk-mpa` carries the committed before-snapshot to the committed after-snapshot by moving
/// `anchor_f_uk_mpa` from 800.0 to 900.0, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_uk_mpa_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-anchor-f-uk-mpa applies to its committed before-snapshot");
    assert_eq!(applied.anchor_f_uk_mpa, 900.0, "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: anchor_f_uk_mpa must read 900.0 after the change");
    assert_eq!(applied, expected_after(), "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: a real 800.0 to 900.0 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-anchor-f-uk-mpa` is its own inverse partner: the inverse step restores `anchor_f_uk_mpa` to its pre-change
/// 800.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_uk_mpa_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-anchor-f-uk-mpa applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-anchor-f-uk-mpa step applies");
        restored = next;
    }
    assert_eq!(restored.anchor_f_uk_mpa, 800.0, "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: the inverse must put anchor_f_uk_mpa back to 800.0");
    assert_eq!(restored, base, "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeAnchorFUkMpa` payload are already canonical:
/// decode then encode is a fixed point, so `anchorFUkMpa` and `newAnchorFUkMpa` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_uk_mpa_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-anchor-f-uk-mpa snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-anchor-f-uk-mpa snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-anchor-f-uk-mpa snapshot reparses");
        assert_eq!(reencoded, original, "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-anchor-f-uk-mpa mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-anchor-f-uk-mpa mutation reparses");
    assert_eq!(reencoded, original, "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: committed mutation JSON is not the canonical externally-tagged ChangeAnchorFUkMpa form carrying newAnchorFUkMpa");
}

/// 🎯️ The declared outcome holds: `change-anchor-f-uk-mpa` at 900.0 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_uk_mpa_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-anchor-f-uk-mpa outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: moving anchor_f_uk_mpa from 800.0 to 900.0 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-anchor-f-uk-mpa` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `anchorFUkMpa` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `anchorFYkMpa`.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_uk_mpa_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().anchor_f_uk_mpa, Some(900.0), "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: the diff must set anchor_f_uk_mpa to 900.0");
    assert!(outcome.diff().artifact.is_none(), "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().anchor_f_yk_mpa.is_none(), "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: change-anchor-f-uk-mpa must leave anchor_f_yk_mpa untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-anchor-f-uk-mpa produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-anchor-f-uk-mpa committed diff decodes");
    assert_eq!(produced, committed, "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `anchorFUkMpa` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-anchor-f-uk-mpa`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_uk_mpa_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-anchor-f-uk-mpa committed diff decodes");
    assert_eq!(decoded.anchor_f_uk_mpa, Some(900.0), "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: the committed diff must carry anchor_f_uk_mpa at 900.0");
    assert!(decoded.selected_check_index.is_none(), "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-anchor-f-uk-mpa committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-anchor-f-uk-mpa committed diff reparses");
    assert_eq!(reencoded, original, "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 800.0 to 900.0 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_uk_mpa_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-anchor-f-uk-mpa committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-anchor-f-uk-mpa committed diff applies to the before-snapshot");
    assert_eq!(produced.anchor_f_uk_mpa, 900.0, "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: the committed diff must leave anchor_f_uk_mpa reading 900.0");
    assert_eq!(produced, expected_after(), "change-anchor-f-uk-mpa/raises-anchor-f-uk-mpa-to-900-0: the committed diff did not carry before to after");
}
