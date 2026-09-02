//! 🧪️ `connect-steps` fixture — `rejects-connecting-a-step-to-itself`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ `SequenceSnapshot` keeps its steps/edges in the composed `s.stdio.semio.flow` CHILD, and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can hand-author —
//! this tree pins the guard branches, which mint nothing.
//!
//! 🔗 `connect-steps` has the deepest guard stack in this vocabulary — source step, target step,
//! duplicate edge id, SELF-LOOP, then already-connected — and this case pins the fourth. The seeded
//! scene holds a single step and no edges, and the committed payload points that step at itself, so
//! the first three guards all pass and the acyclicity invariant is the one that fires.

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

/// 🌱 The committed `⬅️before`, with its composed `content` child resolved to a scene holding the one
/// step the payload names as BOTH endpoints, and no edges at all — so neither endpoint lookup nor
/// the edge-id collision check can be the guard that rejects.
fn before() -> SequenceSnapshot {
    let snapshot: SequenceSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let SequenceMutation::ConnectSteps(payload) = mutation() else {
        panic!("rejects-connecting-a-step-to-itself's committed mutation must be a connect-steps");
    };
    let step = SequenceStep { id: payload.from.clone(), kind: "log.print".into(), params: StepParams::default(), x: 0.0, y: 0.0, slot: None, collapsed: false };
    snapshot.content.set_local_owner(std::sync::Arc::new(SequenceWorkingScene { steps: vec![step], edges: Vec::new() }));
    snapshot
}

/// ▶️ A rejected `connect-steps` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_sequence_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "connect-steps/rejects-connecting-a-step-to-itself: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.content.child_id, &base.content.child_id, "a rejected connect must not mint a new content handle");
}

/// 🚨️ A self-loop is FATAL `mutation.invariant` — a sequence is a DAG, so an edge from a step to
/// itself is an unrepresentable graph, not a miss. The diagnostic addresses the EDGE id, not the
/// step, even though the step id is what makes the edge illegal.
#[semio_framework_async_macros::async_test]
async fn a_self_loop_is_a_fatal_invariant_addressed_at_the_edge() {
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &SequenceDiff::default(), "a rejecting connect-steps must carry the identity diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.invariant", "a self-loop is an invariant breach, not a duplicate id and not a missing target");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "mutation.invariant is Fatal — no merge policy may absorb a self-loop");
    assert_eq!(messages[0].target, vec!["edge-self".to_string()], "the diagnostic addresses the edge the payload tried to create");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[semio_framework_async_macros::async_test]
async fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️.absent must be an empty marker, not a stand-in patch");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &SequenceDiff::default(), "connect-steps/rejects-connecting-a-step-to-itself: a Fatal outcome must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical, and the payload's
/// two endpoints really are the same id — the precondition this whole case rests on.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SequenceSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "connect-steps/rejects-connecting-a-step-to-itself: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "connect-steps/rejects-connecting-a-step-to-itself: committed mutation JSON is not canonical");
    assert_eq!(original.get("from"), original.get("to"), "this case exists to exercise the self-loop guard, so both endpoints must be the same id");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "connect-steps/rejects-connecting-a-step-to-itself declares a rejected outcome");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ↩️ `connect-steps`' inverse is PAYLOAD-derived — a `disconnect-steps` of the edge id it was asked
/// to create, produced even here where no edge was ever created.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_disconnect_of_the_requested_edge_even_when_refused() {
    let inverse = inverse_sequence_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "connect-steps always undoes with exactly one step, got {inverse:?}");
    let SequenceMutation::DisconnectSteps(undo) = &inverse[0] else {
        panic!("connect-steps' inverse must be a disconnect-steps, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "edge-self", "the inverse disconnects exactly the edge id the payload carried");
}

/// 🪪️ The fixture is bound to `connect-steps`' own descriptor, whose entity is the PLURAL `steps`
/// (the relationship), while its address is the singular edge id.
#[semio_framework_async_macros::async_test]
async fn semantics_bind_this_fixture_to_connect_steps() {
    let semantics = <SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("connect", "steps", "connect-steps", "ConnectedSteps"), "the fixture must be bound to connect-steps' own descriptor");
    assert_eq!(<SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::target(&mutation()), vec!["edge-self".to_string()], "connect-steps addresses the edge it creates, never its endpoints");
}
