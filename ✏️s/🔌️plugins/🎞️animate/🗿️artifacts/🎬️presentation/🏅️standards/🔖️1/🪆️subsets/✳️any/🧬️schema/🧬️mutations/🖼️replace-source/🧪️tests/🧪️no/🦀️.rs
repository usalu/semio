//! 🧪️ `replace-source` fixture — `📖️no-ops-when-the-source-is-already-identical`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ `PresentationSnapshot` keeps its `(source, tiles)` in the composed `stdio.semio.presentation`
//! CHILD, and every content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can
//! hand-author — this tree pins the guard branches, which mint nothing.
//!
//! 🖼️ `replace-source` swaps the SINGLETON source facet, and its guard is a whole-`FigureTileSource`
//! value comparison — every field at once, `sourceAspect` and `pdfPage` included. The seeded deck's
//! source is the committed payload's own `newSource`, verbatim, so the comparison is a genuine
//! identity across all five fields rather than a lucky match on `src` alone.

use crate::mutations::{apply_presentation_mutation, inverse_presentation_mutation, PresentationMutation};
use crate::{PresentationDiff, PresentationSnapshot};
use semio_framework_test_fixtures::fixture_text;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️replace-source/🧪️no/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️replace-source/🧪️no/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️replace-source/🧪️no/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️replace-source/🧪️no/🔺️diff/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️replace-source/🧪️no/🎯️outcome/🔣️.json"));

fn mutation() -> PresentationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn expected_after() -> PresentationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `presentation` child resolved to a tile-less deck
/// whose source IS the committed payload's `newSource` — nothing about it is invented.
fn before() -> PresentationSnapshot {
    let snapshot: PresentationSnapshot = semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes");
    let PresentationMutation::ReplaceSource(payload) = mutation() else {
        panic!("no-ops-when-the-source-is-already-identical's committed mutation must be a replace-source");
    };
    assert_eq!(snapshot.source, payload.new_source, "the committed before-snapshot's persisted source must already BE the payload's new source");
    assert!(snapshot.tiles.is_empty(), "the committed before-snapshot is a tile-less deck");
    snapshot
}

/// ▶️ Replacing the source with its own current value carries `before` to exactly the committed
/// `after`, leaving the composed deck handle untouched.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_presentation_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "replace-source/no-ops-when-the-source-is-already-identical: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.presentation.child_id, &base.presentation.child_id, "a source-identity replace must not re-mint the presentation handle");
}

/// 🔺️ The delta is exactly the committed all-null `PresentationDiff`. This matters more here than
/// anywhere else in the tree: source and tiles share ONE composed handle, so a careless
/// source-identity replace would re-mint a deck handle and silently churn the tiles' addressing too.
#[test]
fn produces_committed_diff() {
    let outcome = <PresentationMutation as protocol::ColdMutation<PresentationSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str((*DIFF)).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-source/no-ops-when-the-source-is-already-identical: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(outcome.diff(), &PresentationDiff::default(), "a source-identity replace must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to presentation's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: PresentationDiff = semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str((*DIFF)).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-source/no-ops-when-the-source-is-already-identical: committed diff JSON is not canonical");
}

/// 🩹 The committed identity diff preserves the source, tile list and schema.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: PresentationDiff = semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(decoded, PresentationDiff::default(), "this operation must carry the identity diff");
    let produced = protocol::apply_diff_cold(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-source/no-ops-when-the-source-is-already-identical: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical — `sourceAspect` is
/// presentation, `pdfPage` is omitted: both are `skip_serializing_if = "Option::is_none"`, so an unset
/// page must not appear as null.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: PresentationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-source/no-ops-when-the-source-is-already-identical: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str((*MUTATION)).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-source/no-ops-when-the-source-is-already-identical: committed mutation JSON is not canonical");
    let source = original.get("ReplaceSource").and_then(|payload| payload.get("newSource")).expect("the payload carries a whole source");
    assert!(source.get("sourceAspect").is_some() && source.get("pdfPage").is_none(), "an unset optional source field is omitted, never null");
}

/// 🎯️ The declared outcome holds: `no-op`, with one untargeted Warning `mutation.no-op`. Like
/// `replace-tiles`, this singleton verb has no addressable target and therefore no error branch.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str((*OUTCOME)).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("no-op"), "replace-source/no-ops-when-the-source-is-already-identical declares a no-op outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <PresentationMutation as protocol::ColdMutation<PresentationSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Warning, "an unchanged source is a warning; replace-source has no error branch");
}

/// ↩️ A no-op leaves undo history empty and preserves the full snapshot.
#[test]
fn unchanged_source_has_an_empty_inverse() {
    let base = before();
    let operation = mutation();
    let inverse = inverse_presentation_mutation(&base, &operation).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty());
    assert_eq!(apply_presentation_mutation(&base, &operation).expect("no-op applies"), base);
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
