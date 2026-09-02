//! 🧪️ `duplicate-step` fixture — `rejects-when-the-new-id-already-exists`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ `SequenceSnapshot` keeps its steps/edges in the composed `s.stdio.semio.flow` CHILD, and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can hand-author —
//! this tree pins the guard branches, which mint nothing.
//!
//! 🧬 `duplicate-step` is the only sequence verb that reads one id and writes another, so it guards
//! BOTH: an Error if the SOURCE is missing, then a Fatal if the NEW id already exists. This case
//! pins the second — the seeded scene holds the source AND a step already occupying the requested
//! new id, so the source lookup genuinely succeeds before the collision rejects.

use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::{apply_sequence_mutation, inverse_sequence_mutation, SequenceMutation};
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStep, SequenceWorkingScene, StepParams};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> SequenceMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> SequenceSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `content` child resolved to a two-step scene: the
/// source the payload copies FROM, and a step already holding the id the payload wants to copy TO.
/// Both ids come straight from the committed payload.
fn before() -> SequenceSnapshot {
    let snapshot: SequenceSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let SequenceMutation::DuplicateStep(payload) = mutation() else {
        panic!("rejects-when-the-new-id-already-exists's committed mutation must be a duplicate-step");
    };
    let step = |id: String| SequenceStep { id, kind: "log.print".into(), params: StepParams::default(), x: 0.0, y: 0.0, slot: None, collapsed: false };
    snapshot.content.set_local_owner(std::sync::Arc::new(SequenceWorkingScene { steps: vec![step(payload.source_id.clone()), step(payload.new_id.clone())], edges: Vec::new() }));
    snapshot
}

/// ▶️ A rejected `duplicate-step` leaves the document byte-identical to the committed `after` — the
/// step already occupying the requested id is not overwritten.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_sequence_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "duplicate-step/rejects-when-the-new-id-already-exists: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.content.child_id, &base.content.child_id, "a rejected duplicate must not mint a new content handle");
}

/// 🚨️ The collision is FATAL `mutation.duplicate-id` and it is addressed at the NEW id, not the
/// source — the one place in this vocabulary where the diagnostic's target is the id the payload
/// wanted to write rather than the one it read.
#[semio_framework_async_macros::async_test]
async fn a_taken_new_id_is_fatal_and_addressed_at_the_new_id() {
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &SequenceDiff::default(), "a rejecting duplicate-step must carry the identity diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "a taken new id is reported as duplicate-id, not target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "duplicate-id is Fatal — no merge policy may absorb it");
    assert_eq!(messages[0].target, vec!["step-copy".to_string()], "the diagnostic names the destination id, not the source the payload also carries");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[semio_framework_async_macros::async_test]
async fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️.absent must be an empty marker, not a stand-in patch");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &SequenceDiff::default(), "duplicate-step/rejects-when-the-new-id-already-exists: a Fatal outcome must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical, and the payload's
/// two ids really differ — otherwise the source guard, not the collision guard, would be the one
/// under test.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SequenceSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "duplicate-step/rejects-when-the-new-id-already-exists: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "duplicate-step/rejects-when-the-new-id-already-exists: committed mutation JSON is not canonical");
    assert_ne!(original.get("sourceId"), original.get("newId"), "the copy must address a different id from its source");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "duplicate-step/rejects-when-the-new-id-already-exists declares a rejected outcome");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ↩️ `duplicate-step`'s inverse is PAYLOAD-derived — a `delete-step` of the new id — and it is
/// produced unconditionally. Here that is a genuinely sharp edge worth pinning: the undo names a
/// step that ALREADY EXISTED and that this refused mutation never created, so replaying it blindly
/// would destroy pre-existing content.
#[semio_framework_async_macros::async_test]
async fn inverse_targets_the_new_id_even_though_that_step_predates_this_mutation() {
    let base = before();
    let inverse = inverse_sequence_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "duplicate-step always undoes with exactly one step, got {inverse:?}");
    let SequenceMutation::DeleteStep(undo) = &inverse[0] else {
        panic!("duplicate-step's inverse must be a delete-step, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "step-copy", "the inverse deletes the requested new id, never the source");
    assert!(crate::artifacts::sequence::sequence_working_scene(&base).steps.iter().any(|step| step.id == undo.id), "the id the inverse targets is a step that existed before this mutation was ever attempted");
}

/// 🪪️ The fixture is bound to `duplicate-step`'s own descriptor, whose address is the NEW id — the
/// thing the verb brings into existence, matching the diagnostic's own target.
#[semio_framework_async_macros::async_test]
async fn semantics_bind_this_fixture_to_duplicate_step() {
    let semantics = <SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("duplicate", "step", "duplicate-step", "DuplicatedStep"), "the fixture must be bound to duplicate-step's own descriptor");
    assert_eq!(<SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::target(&mutation()), vec!["step-copy".to_string()], "duplicate-step addresses the copy it creates");
}
