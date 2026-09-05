//! 🧪️ `change-uri` fixture — `🛟️republishes-the-brief-under-a-new-uri`.
//!
//! `change-uri` moves the document's ADDRESSABLE LOCATION. Its diff oracle has exactly one guard
//! (`base.uri == new_uri` ⇒ Warning `mutation.no-op`) and otherwise builds a one-field
//! `WriterDiff { uri: Some(..) }` straight from the payload. Crucially it does NOT re-derive `id`
//! from the new uri's last path segment — that derivation belongs to the app's `open_document`
//! handler, not to this mutation — so the identity field must survive the move untouched.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use crate::artifacts::writer::schema::mutations::{apply_writer_mutation, inverse_writer_mutation, WriterMutation};
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> WriterSnapshot {
    serde_json::from_str(BEFORE).expect("before writer document decodes")
}
fn expected_after() -> WriterSnapshot {
    serde_json::from_str(AFTER).expect("after writer document decodes")
}
fn mutation() -> WriterMutation {
    serde_json::from_str(MUTATION).expect("change-uri mutation decodes")
}

/// ▶️ Moving the brief from `writer://drafts/` to `writer://published/` rewrites `uri` alone.
#[semio_framework_async_macros::async_test]
async fn relocates_only_the_uri() {
    let mut snapshot = before();
    apply_writer_mutation(&mut snapshot, &mutation()).expect("change-uri applies to its committed before-document");
    assert_eq!(snapshot, expected_after(), "change-uri/republishes-the-brief-under-a-new-uri: the relocated document differs from the committed after-snapshot");
    assert_eq!(snapshot.id, before().id, "change-uri/republishes-the-brief-under-a-new-uri: changing the uri must not re-derive the document identity");
}

/// ↩️ `change-uri`'s inverse reads `base.uri` — never the diff — so undoing puts the brief back
/// under `writer://drafts/brief.md`.
#[semio_framework_async_macros::async_test]
async fn relocating_back_restores_before() {
    let base = before();
    let inverse = inverse_writer_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "change-uri/republishes-the-brief-under-a-new-uri: a document uri always has a prior value, so exactly one undo step");
    let mut snapshot = base.clone();
    apply_writer_mutation(&mut snapshot, &mutation()).expect("forward change-uri applies");
    for step in &inverse {
        apply_writer_mutation(&mut snapshot, step).expect("the change-uri inverse step applies");
    }
    assert_eq!(snapshot, base, "change-uri/republishes-the-brief-under-a-new-uri: relocating back to the drafts uri did not restore the before-document");
}

/// 🔣️ Both committed documents and the `changeUri` payload are canonical — note `uri` is emitted
/// even though it carries `#[serde(default = "default_uri")]`, since it has no skip attribute.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WriterSnapshot = serde_json::from_str(text).expect("writer document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("writer document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("writer document reparses");
        assert_eq!(reencoded, original, "change-uri/republishes-the-brief-under-a-new-uri: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("changeUri payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("changeUri payload reparses");
    assert_eq!(reencoded, original, "change-uri/republishes-the-brief-under-a-new-uri: committed changeUri JSON is not canonical");
}

/// 🎯️ The published uri differs from the draft uri, so the single `mutation.no-op` guard does not
/// fire and the declared `applied` outcome must be message-free.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-uri/republishes-the-brief-under-a-new-uri: this fixture declares an applied outcome");
    let produced = <WriterMutation as protocol::Mutation<WriterSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "change-uri/republishes-the-brief-under-a-new-uri: a genuinely new uri must not raise mutation.no-op");
    assert!(produced.messages().is_empty(), "change-uri/republishes-the-brief-under-a-new-uri: an accepted relocation emits no diagnostics");
}

/// 🔺️ Exactly one of `WriterDiff`'s sixteen fields may be set here — `uri`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <WriterMutation as protocol::Mutation<WriterSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced change-uri diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-uri/republishes-the-brief-under-a-new-uri: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `WriterDiff` and re-encodes unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: WriterDiff = serde_json::from_str(DIFF).expect("committed change-uri diff decodes");
    assert_eq!(decoded.uri.as_deref(), Some("writer://published/brief.md"), "change-uri/republishes-the-brief-under-a-new-uri: the committed diff must set the new uri");
    assert!(decoded.id.is_none(), "change-uri/republishes-the-brief-under-a-new-uri: relocating must not smuggle an identity change into the same diff");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-uri/republishes-the-brief-under-a-new-uri: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: WriterDiff = serde_json::from_str(DIFF).expect("committed change-uri diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "change-uri/republishes-the-brief-under-a-new-uri: committed diff did not carry before to after");
}
