//! 🧪️ `replace-tiles` fixture — `no-ops-when-the-collection-is-already-empty`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ `PresentSnapshot` keeps its `(source, tiles)` in the composed `s.stdio.semio.presentation`
//! CHILD, and every content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can
//! hand-author — this tree pins the guard branches, which mint nothing.
//!
//! 🔁 `replace-tiles` is the whole-COLLECTION swap, and an empty `new_tiles` is its "clear the deck"
//! gesture — the reason no separate `clear-tiles` verb exists. This case pins the boundary where
//! that gesture is a genuine no-op: an unseeded `presentation` handle reads back as the default
//! source with NO tiles, so clearing an already-empty collection must produce nothing at all rather
//! than re-minting an identical deck. Nothing is seeded here; the emptiness IS the fallback.

use crate::artifacts::present::mutations::{apply_present_mutation, inverse_present_mutation, PresentMutation};
use crate::artifacts::present::{PresentDiff, PresentSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> PresentMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn before() -> PresentSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> PresentSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// ▶️ Clearing an already-empty deck carries `before` to exactly the committed `after`, leaving the
/// composed deck handle untouched.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_present_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "replace-tiles/no-ops-when-the-collection-is-already-empty: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.presentation.child_id, &base.presentation.child_id, "a collection-identity replace must not re-mint the presentation handle");
}

/// 🔺️ The delta is exactly the committed all-null `PresentDiff` — `replace-tiles` compares the
/// WHOLE collection by value before building anything, so a wholesale swap for an equal collection
/// never reaches `diff_set_presentation`.
#[test]
fn produces_committed_diff() {
    let outcome = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-tiles/no-ops-when-the-collection-is-already-empty: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(outcome.diff(), &PresentDiff::default(), "a collection-identity replace must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to present's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: PresentDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-tiles/no-ops-when-the-collection-is-already-empty: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`, with the
/// deck slot never set.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: PresentDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.presentation.is_none(), "a collection-identity replace must leave the composed deck slot unset");
    let produced = <PresentDiff as protocol::MutationDiff<PresentSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-tiles/no-ops-when-the-collection-is-already-empty: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical — `newTiles` is a
/// real empty array, the "clear" gesture, not an omitted field.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PresentSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-tiles/no-ops-when-the-collection-is-already-empty: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-tiles/no-ops-when-the-collection-is-already-empty: committed mutation JSON is not canonical");
    assert_eq!(original.get("ReplaceTiles").and_then(|payload| payload.get("newTiles")).and_then(serde_json::Value::as_array).map(Vec::len), Some(0), "the clear gesture sends an explicit empty collection");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op`.
/// `replace-tiles` addresses the collection as a whole, so it has no missing-target branch at all —
/// this warning is its ONLY diagnostic.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-tiles/no-ops-when-the-collection-is-already-empty declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "an unchanged collection is a warning; replace-tiles has no error branch");
}

/// ↩️ `replace-tiles`' inverse is unconditional and BASE-derived: one `replace-tiles` carrying the
/// pre-state collection wholesale — here the empty collection — never a per-tile plan.
#[test]
fn inverse_restores_the_whole_base_collection() {
    let base = before();
    let inverse = inverse_present_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "replace-tiles always undoes with exactly one wholesale step, got {inverse:?}");
    let PresentMutation::ReplaceTiles(undo) = &inverse[0] else {
        panic!("replace-tiles' inverse must be a replace-tiles, got {:?}", inverse[0]);
    };
    assert!(undo.new_tiles.is_empty(), "the inverse restores the captured (empty) base collection");
    let restored = apply_present_mutation(&apply_present_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "replace-tiles/no-ops-when-the-collection-is-already-empty: inverse did not restore the before-snapshot");
}
