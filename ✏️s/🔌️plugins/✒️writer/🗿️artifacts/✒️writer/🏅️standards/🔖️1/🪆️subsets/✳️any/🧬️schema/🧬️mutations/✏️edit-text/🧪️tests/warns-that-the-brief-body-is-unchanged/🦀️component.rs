//! 🧪️ `edit-text` fixture — `warns-that-the-brief-body-is-unchanged`.
//!
//! `edit-text` is the ONLY writer mutation that reaches the composed `s.stdio.semio.document`
//! child: on a real edit it hands off to `diff_set_text`, which mints a fresh content-addressed
//! handle and seeds the working-scene cache. Its diff oracle guards that with an equality check
//! against the CURRENT body (`writer_text(base) == payload.text` ⇒ Warning `mutation.no-op`), and
//! this case drives exactly that guard: resending the brief's own body must produce an empty diff
//! and, above all, must NOT re-mint the handle — a spurious re-mint would be invisible in the text
//! but would rewrite the document's content address on every keystroke-free save.
//!
//! 🕸️ Because the handle is content-addressed and writer's working scene is a thread-local scratch
//! cache, the committed `⬅️before` carries the handle and this file caches that handle's body; the
//! two together ARE the before-state.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use crate::artifacts::writer::schema::mutations::{apply_writer_mutation, inverse_writer_mutation, WriterMutation};
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

/// 📝 The body the committed `document` handle stands for — byte-identical to the payload's own
/// `text`, which is the whole point of this fixture.
const CACHED_BODY: &str = "# Mission Brief\n\nHold the current draft.\n";

fn before() -> WriterSnapshot {
    let snapshot: WriterSnapshot = serde_json::from_str(BEFORE).expect("before writer document decodes");
    crate::artifacts::writer::cache_writer_document_text(&snapshot.document.child_id, CACHED_BODY);
    snapshot
}
fn expected_after() -> WriterSnapshot {
    serde_json::from_str(AFTER).expect("after writer document decodes")
}
fn mutation() -> WriterMutation {
    serde_json::from_str(MUTATION).expect("edit-text mutation decodes")
}

/// ▶️ Resending the same body is accepted and changes nothing — most importantly the
/// content-addressed `document` child handle is the very same handle afterwards.
#[semio_framework_async_macros::async_test]
async fn the_idempotent_edit_leaves_the_document_handle_alone() {
    let base = before();
    let mut snapshot = base.clone();
    apply_writer_mutation(&mut snapshot, &mutation()).expect("edit-text applies to its committed before-document");
    assert_eq!(snapshot, expected_after(), "edit-text/warns-that-the-brief-body-is-unchanged: the no-op edit must reproduce the committed after-snapshot");
    assert_eq!(snapshot.document.child_id, base.document.child_id, "edit-text/warns-that-the-brief-body-is-unchanged: an unchanged body must not mint a new content address");
}

/// ↩️ `edit-text`'s inverse reads the body off BASE's own handle through the working-scene cache —
/// never the diff — so here it hands back the identical body the payload already carried.
#[semio_framework_async_macros::async_test]
async fn the_inverse_resends_the_identical_body() {
    let base = before();
    let inverse = inverse_writer_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "edit-text/warns-that-the-brief-body-is-unchanged: undoing a body edit is exactly one body edit back");
    let WriterMutation::EditText(undo) = &inverse[0] else {
        panic!("edit-text/warns-that-the-brief-body-is-unchanged: edit-text's inverse must be an edit-text");
    };
    assert_eq!(undo.text, CACHED_BODY, "edit-text/warns-that-the-brief-body-is-unchanged: the undo must carry the body cached behind the before-handle");
    let mut snapshot = base.clone();
    apply_writer_mutation(&mut snapshot, &mutation()).expect("forward edit-text applies");
    for step in &inverse {
        apply_writer_mutation(&mut snapshot, step).expect("the edit-text inverse step applies");
    }
    assert_eq!(snapshot, base, "edit-text/warns-that-the-brief-body-is-unchanged: undoing a no-op must still land back on the before-document");
}

/// 🔣️ Both committed documents and the `editText` payload are canonical — the payload's `text`
/// carries real newline escapes and must survive the JSON round trip unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WriterSnapshot = serde_json::from_str(text).expect("writer document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("writer document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("writer document reparses");
        assert_eq!(reencoded, original, "edit-text/warns-that-the-brief-body-is-unchanged: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("editText payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("editText payload reparses");
    assert_eq!(reencoded, original, "edit-text/warns-that-the-brief-body-is-unchanged: committed editText JSON is not canonical");
}

/// 🎯️ An unchanged body is `applied` with a single Warning — `edit-text` has no target to miss, so
/// `mutation.no-op` is the only diagnostic this oracle can ever raise.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-text/warns-that-the-brief-body-is-unchanged: a no-op is applied, not rejected");
    let produced = <WriterMutation as protocol::Mutation<WriterSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), Some(protocol::Severity::Warning), "edit-text/warns-that-the-brief-body-is-unchanged: an unchanged body is a Warning, never an Error");
    assert_eq!(produced.messages().len(), 1, "edit-text/warns-that-the-brief-body-is-unchanged: exactly one diagnostic is raised");
    assert_eq!(produced.messages()[0].code.0.as_str(), declared["messages"][0]["code"].as_str().expect("declared message code is a string"), "edit-text/warns-that-the-brief-body-is-unchanged: raised diagnostic code differs from the declared one");
}

/// 🔺️ The committed diff is `WriterDiff`'s all-null default: the oracle returns before it ever
/// reaches `diff_set_text`, so `document` — this mutation's ONLY output field — stays null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <WriterMutation as protocol::Mutation<WriterSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced edit-text diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-text/warns-that-the-brief-body-is-unchanged: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `WriterDiff` and re-encodes unchanged — all sixteen fields are
/// emitted as `null` because none carries `skip_serializing_if`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: WriterDiff = serde_json::from_str(DIFF).expect("committed edit-text diff decodes");
    assert_eq!(decoded, WriterDiff::default(), "edit-text/warns-that-the-brief-body-is-unchanged: a no-op's committed diff must be the type's own default");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-text/warns-that-the-brief-body-is-unchanged: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document — trivially, but
/// it must still be the committed diff that does it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: WriterDiff = serde_json::from_str(DIFF).expect("committed edit-text diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "edit-text/warns-that-the-brief-body-is-unchanged: committed diff did not carry before to after");
}
