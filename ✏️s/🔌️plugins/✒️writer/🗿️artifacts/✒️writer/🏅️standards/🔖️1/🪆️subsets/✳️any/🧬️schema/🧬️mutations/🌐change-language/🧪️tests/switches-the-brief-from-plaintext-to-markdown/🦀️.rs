//! 🧪️ `change-language` fixture — `switches-the-brief-from-plaintext-to-markdown`.
//!
//! `change-language` sets the document's LANGUAGE MODE. Its diff oracle has exactly one guard
//! (`base.language_id == new_language_id` ⇒ Warning `mutation.no-op`) and otherwise builds a
//! one-field `WriterDiff { language_id: Some(..) }` straight from the payload. `language_id` is
//! also an input to `document_child_handle`'s content hash, but this mutation deliberately does
//! NOT re-mint the composed `document` handle — only `edit-text` ever does that.
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
    serde_json::from_str(MUTATION).expect("change-language mutation decodes")
}

/// ▶️ Switching the brief to `markdown` rewrites `languageId` alone — the document child handle,
/// whose content hash also folds in a language id, is explicitly left as it was.
#[semio_framework_async_macros::async_test]
async fn switches_only_the_language_mode() {
    let mut snapshot = before();
    apply_writer_mutation(&mut snapshot, &mutation()).expect("change-language applies to its committed before-document");
    assert_eq!(snapshot, expected_after(), "change-language/switches-the-brief-from-plaintext-to-markdown: the reclassified document differs from the committed after-snapshot");
    assert_eq!(snapshot.document, before().document, "change-language/switches-the-brief-from-plaintext-to-markdown: changing the language mode must not re-mint the document child handle");
}

/// ↩️ `change-language`'s inverse reads `base.language_id` — never the diff — so undoing puts the
/// brief back on `plaintext`.
#[semio_framework_async_macros::async_test]
async fn switching_back_restores_before() {
    let base = before();
    let inverse = inverse_writer_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "change-language/switches-the-brief-from-plaintext-to-markdown: a document language always has a prior value, so exactly one undo step");
    let mut snapshot = base.clone();
    apply_writer_mutation(&mut snapshot, &mutation()).expect("forward change-language applies");
    for step in &inverse {
        apply_writer_mutation(&mut snapshot, step).expect("the change-language inverse step applies");
    }
    assert_eq!(snapshot, base, "change-language/switches-the-brief-from-plaintext-to-markdown: switching back to plaintext did not restore the before-document");
}

/// 🔣️ Both committed documents and the `changeLanguage` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WriterSnapshot = serde_json::from_str(text).expect("writer document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("writer document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("writer document reparses");
        assert_eq!(reencoded, original, "change-language/switches-the-brief-from-plaintext-to-markdown: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("changeLanguage payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("changeLanguage payload reparses");
    assert_eq!(reencoded, original, "change-language/switches-the-brief-from-plaintext-to-markdown: committed changeLanguage JSON is not canonical");
}

/// 🎯️ `markdown` differs from the base `plaintext`, so the single `mutation.no-op` guard does not
/// fire and the declared `applied` outcome must be message-free.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-language/switches-the-brief-from-plaintext-to-markdown: this fixture declares an applied outcome");
    let produced = <WriterMutation as protocol::Mutation<WriterSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "change-language/switches-the-brief-from-plaintext-to-markdown: a genuinely new language id must not raise mutation.no-op");
    assert!(produced.messages().is_empty(), "change-language/switches-the-brief-from-plaintext-to-markdown: an accepted language switch emits no diagnostics");
}

/// 🔺️ Exactly one of `WriterDiff`'s sixteen fields may be set here — `languageId`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <WriterMutation as protocol::Mutation<WriterSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced change-language diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-language/switches-the-brief-from-plaintext-to-markdown: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `WriterDiff` and re-encodes unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: WriterDiff = serde_json::from_str(DIFF).expect("committed change-language diff decodes");
    assert_eq!(decoded.language_id.as_deref(), Some("markdown"), "change-language/switches-the-brief-from-plaintext-to-markdown: the committed diff must set the new language id");
    assert!(decoded.document.is_none(), "change-language/switches-the-brief-from-plaintext-to-markdown: no replacement document handle may ride along with a language switch");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-language/switches-the-brief-from-plaintext-to-markdown: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: WriterDiff = serde_json::from_str(DIFF).expect("committed change-language diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "change-language/switches-the-brief-from-plaintext-to-markdown: committed diff did not carry before to after");
}
