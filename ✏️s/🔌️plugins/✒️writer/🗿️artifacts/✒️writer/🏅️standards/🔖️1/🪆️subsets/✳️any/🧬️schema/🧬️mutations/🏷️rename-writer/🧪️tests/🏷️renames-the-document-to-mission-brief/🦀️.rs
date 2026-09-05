//! 🧪️ `rename-writer` fixture — `🏷️renames-the-document-to-mission-brief`.
//!
//! `rename-writer` touches the document's IDENTITY field only. Its diff oracle has exactly one
//! guard (`base.id == new_id` ⇒ Warning `mutation.no-op`) and otherwise builds a one-field
//! `WriterDiff { id: Some(..) }` straight from the payload — never an apply-then-capture, and in
//! particular never a re-mint of the content-addressed `document` child handle.
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
    serde_json::from_str(MUTATION).expect("rename-writer mutation decodes")
}

/// ▶️ Renaming `brief` to `mission-brief` rewrites `id` and nothing else — `uri`, `languageId` and
/// the composed `document` handle all ride through untouched.
#[semio_framework_async_macros::async_test]
async fn renames_only_the_identity_field() {
    let mut snapshot = before();
    apply_writer_mutation(&mut snapshot, &mutation()).expect("rename-writer applies to its committed before-document");
    assert_eq!(snapshot, expected_after(), "rename-writer/renames-the-document-to-mission-brief: the renamed document differs from the committed after-snapshot");
    assert_eq!(
        (snapshot.uri.as_str(), snapshot.document.child_id.as_str()),
        (before().uri.as_str(), before().document.child_id.as_str()),
        "rename-writer/renames-the-document-to-mission-brief: renaming must not move the uri or re-mint the document child handle"
    );
}

/// ↩️ `rename-writer`'s inverse reads `base.id` — never the diff — so undoing restores `brief`.
#[semio_framework_async_macros::async_test]
async fn renaming_back_restores_before() {
    let base = before();
    let inverse = inverse_writer_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "rename-writer/renames-the-document-to-mission-brief: a document identity always has a prior value, so exactly one undo step");
    let mut snapshot = base.clone();
    apply_writer_mutation(&mut snapshot, &mutation()).expect("forward rename-writer applies");
    for step in &inverse {
        apply_writer_mutation(&mut snapshot, step).expect("the rename-writer inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-writer/renames-the-document-to-mission-brief: renaming back to \"brief\" did not restore the before-document");
}

/// 🔣️ Both committed documents and the `renameWriter` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WriterSnapshot = serde_json::from_str(text).expect("writer document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("writer document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("writer document reparses");
        assert_eq!(reencoded, original, "rename-writer/renames-the-document-to-mission-brief: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("renameWriter payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("renameWriter payload reparses");
    assert_eq!(reencoded, original, "rename-writer/renames-the-document-to-mission-brief: committed renameWriter JSON is not canonical");
}

/// 🎯️ `mission-brief` differs from the base id, so the single `mutation.no-op` guard does not
/// fire and the declared `applied` outcome must be message-free.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-writer/renames-the-document-to-mission-brief: this fixture declares an applied outcome");
    let produced = <WriterMutation as protocol::Mutation<WriterSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "rename-writer/renames-the-document-to-mission-brief: a genuinely new id must not raise mutation.no-op");
    assert!(produced.messages().is_empty(), "rename-writer/renames-the-document-to-mission-brief: an accepted rename emits no diagnostics");
}

/// 🔺️ `WriterDiff` has sixteen optional fields across the artifact, presence and config lanes;
/// this mutation is allowed to set exactly one of them — `id`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <WriterMutation as protocol::Mutation<WriterSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced rename-writer diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-writer/renames-the-document-to-mission-brief: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `WriterDiff` and re-encodes unchanged — including the fifteen
/// nulls, which `WriterDiff` emits because no field carries `skip_serializing_if`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: WriterDiff = serde_json::from_str(DIFF).expect("committed rename-writer diff decodes");
    assert_eq!(decoded.id.as_deref(), Some("mission-brief"), "rename-writer/renames-the-document-to-mission-brief: the committed diff must set the new identity");
    assert!(decoded.document.is_none() && decoded.artifact.is_none(), "rename-writer/renames-the-document-to-mission-brief: a rename must neither replace the document child nor swap the whole artifact");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-writer/renames-the-document-to-mission-brief: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: WriterDiff = serde_json::from_str(DIFF).expect("committed rename-writer diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "rename-writer/renames-the-document-to-mission-brief: committed diff did not carry before to after");
}
