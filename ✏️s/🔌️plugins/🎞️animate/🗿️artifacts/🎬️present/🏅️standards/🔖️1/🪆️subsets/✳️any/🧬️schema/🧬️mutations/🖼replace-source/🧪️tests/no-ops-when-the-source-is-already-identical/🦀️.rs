//! 🧪️ `replace-source` fixture — `no-ops-when-the-source-is-already-identical`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ `PresentSnapshot` keeps its `(source, tiles)` in the composed `s.stdio.semio.presentation`
//! CHILD, and every content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can
//! hand-author — this tree pins the guard branches, which mint nothing.
//!
//! 🖼️ `replace-source` swaps the SINGLETON source facet, and its guard is a whole-`FigureTileSource`
//! value comparison — every field at once, `sourceAspect` and `pdfPage` included. The seeded deck's
//! source is the committed payload's own `newSource`, verbatim, so the comparison is a genuine
//! identity across all five fields rather than a lucky match on `src` alone.

use crate::artifacts::present::mutations::{apply_present_mutation, inverse_present_mutation, PresentMutation};
use crate::artifacts::present::{cache_present_working_scene, PresentDiff, PresentSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> PresentMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> PresentSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `presentation` child resolved to a tile-less deck
/// whose source IS the committed payload's `newSource` — nothing about it is invented.
fn before() -> PresentSnapshot {
    let snapshot: PresentSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let PresentMutation::ReplaceSource(payload) = mutation() else {
        panic!("no-ops-when-the-source-is-already-identical's committed mutation must be a replace-source");
    };
    cache_present_working_scene(&snapshot.presentation.child_id, &payload.new_source, &[]);
    snapshot
}

/// ▶️ Replacing the source with its own current value carries `before` to exactly the committed
/// `after`, leaving the composed deck handle untouched.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_present_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "replace-source/no-ops-when-the-source-is-already-identical: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.presentation.child_id, &base.presentation.child_id, "a source-identity replace must not re-mint the presentation handle");
}

/// 🔺️ The delta is exactly the committed all-null `PresentDiff`. This matters more here than
/// anywhere else in the tree: source and tiles share ONE composed handle, so a careless
/// source-identity replace would re-mint a deck handle and silently churn the tiles' addressing too.
#[test]
fn produces_committed_diff() {
    let outcome = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-source/no-ops-when-the-source-is-already-identical: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(outcome.diff(), &PresentDiff::default(), "a source-identity replace must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to present's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: PresentDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-source/no-ops-when-the-source-is-already-identical: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`, with the
/// shared deck slot never set.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: PresentDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.presentation.is_none(), "a source-identity replace must leave the shared deck slot unset");
    let produced = <PresentDiff as protocol::MutationDiff<PresentSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-source/no-ops-when-the-source-is-already-identical: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical — `sourceAspect` is
/// present, `pdfPage` is omitted: both are `skip_serializing_if = "Option::is_none"`, so an unset
/// page must not appear as null.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PresentSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-source/no-ops-when-the-source-is-already-identical: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-source/no-ops-when-the-source-is-already-identical: committed mutation JSON is not canonical");
    let source = original.get("ReplaceSource").and_then(|payload| payload.get("newSource")).expect("the payload carries a whole source");
    assert!(source.get("sourceAspect").is_some() && source.get("pdfPage").is_none(), "an unset optional source field is omitted, never null");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op`. Like
/// `replace-tiles`, this singleton verb has no addressable target and therefore no error branch.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-source/no-ops-when-the-source-is-already-identical declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "an unchanged source is a warning; replace-source has no error branch");
}

/// ↩️ `replace-source`'s inverse ignores its payload entirely and rebuilds from the BASE source, so
/// here it is byte-identical to the forward payload — a value-identical replace is its own inverse.
#[test]
fn inverse_restores_the_whole_base_source() {
    let base = before();
    let PresentMutation::ReplaceSource(payload) = mutation() else {
        panic!("committed mutation must be a replace-source");
    };
    let inverse = inverse_present_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "replace-source always undoes with exactly one wholesale step, got {inverse:?}");
    let PresentMutation::ReplaceSource(undo) = &inverse[0] else {
        panic!("replace-source's inverse must be a replace-source, got {:?}", inverse[0]);
    };
    assert_eq!(undo.new_source, payload.new_source, "the inverse restores the captured base source, which here equals the requested one");
    let restored = apply_present_mutation(&apply_present_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "replace-source/no-ops-when-the-source-is-already-identical: inverse did not restore the before-snapshot");
}
