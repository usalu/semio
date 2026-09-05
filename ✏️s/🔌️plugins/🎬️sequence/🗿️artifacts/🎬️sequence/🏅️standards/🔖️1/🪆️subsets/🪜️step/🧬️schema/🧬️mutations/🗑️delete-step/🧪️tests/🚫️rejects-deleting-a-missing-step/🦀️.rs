//! 🧪️ `delete-step` fixture — `🚫️rejects-deleting-a-missing-step`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ `SequenceSnapshot` keeps its steps/edges in the composed `s.stdio.semio.flow` CHILD, and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can hand-author —
//! this tree pins the guard branches, which mint nothing.
//!
//! 🗑️ `delete-step` is the one sequence verb with an edge CASCADE: deleting a step also severs every
//! edge touching it and reports that as an Info `mutation.cascade`. This case pins the branch BEFORE
//! the cascade — the committed `content` handle is left unseeded, so the scene is empty, the target
//! guard fires first, and no cascade note is emitted at all.

use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::{apply_sequence_mutation, inverse_sequence_mutation, SequenceMutation};
use crate::artifacts::sequence::SequenceSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> SequenceMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn before() -> SequenceSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> SequenceSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// ▶️ A rejected `delete-step` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_sequence_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "delete-step/rejects-deleting-a-missing-step: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.content.child_id, &base.content.child_id, "a rejected delete must not mint a new content handle");
}

/// 🚫️ A missing step is an Error `mutation.target-missing`, and — the point of this case — it is the
/// ONLY diagnostic: the edge-cascade Info note must not be emitted for a delete that never happened.
#[semio_framework_async_macros::async_test]
async fn a_missing_step_is_an_error_with_no_cascade_note() {
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &SequenceDiff::default(), "a rejecting delete-step must carry the identity diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "a rejected delete emits its error and nothing else, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing step is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing delete target is an Error, never Fatal");
    assert!(messages.iter().all(|message| message.code.0 != "mutation.cascade"), "no edges were severed, so no cascade note may be reported");
    assert_eq!(messages[0].target, vec!["step-gone".to_string()], "the diagnostic addresses the missing step id");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[semio_framework_async_macros::async_test]
async fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️.absent must be an empty marker, not a stand-in patch");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &SequenceDiff::default(), "delete-step/rejects-deleting-a-missing-step: a rejection must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SequenceSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-step/rejects-deleting-a-missing-step: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-step/rejects-deleting-a-missing-step: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "delete-step/rejects-deleting-a-missing-step declares a rejected outcome");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ↩️ `delete-step`'s inverse is the most elaborate in this vocabulary — it replays the WHOLE scene
/// (delete every step, re-create every step in order, then re-connect every edge) so that step and
/// edge order survive the undo. A missing target short-circuits all of that to nothing.
#[semio_framework_async_macros::async_test]
async fn inverse_of_a_missing_delete_is_empty() {
    let inverse = inverse_sequence_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "delete-step's scene-replay inverse must not run at all when its target is absent, got {inverse:?}");
}

/// 🪪️ The fixture is bound to `delete-step`'s own descriptor and its single-segment address.
#[semio_framework_async_macros::async_test]
async fn semantics_bind_this_fixture_to_delete_step() {
    let semantics = <SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "step", "delete-step", "DeletedStep"), "the fixture must be bound to delete-step's own descriptor");
    assert_eq!(<SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::target(&mutation()), vec!["step-gone".to_string()], "delete-step addresses exactly the step id");
}
