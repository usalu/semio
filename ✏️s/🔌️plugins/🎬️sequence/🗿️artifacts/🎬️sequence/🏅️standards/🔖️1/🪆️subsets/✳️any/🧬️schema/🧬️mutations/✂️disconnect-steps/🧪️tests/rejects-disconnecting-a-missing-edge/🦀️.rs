//! 🧪️ `disconnect-steps` fixture — `rejects-disconnecting-a-missing-edge`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ `SequenceSnapshot` keeps its steps/edges in the composed `s.stdio.semio.flow` CHILD, and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can hand-author —
//! this tree pins the guard branches, which mint nothing.
//!
//! ✂️ `disconnect-steps` is the only sequence verb addressed at an EDGE rather than a step, and its
//! single guard searches `scene.edges`, never `scene.steps`. The committed `content` handle is left
//! unseeded, so the scene has no edges and that guard fires on an id that is an edge id, not a step
//! id — the distinction the diagnostic's own message spells out.

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

/// ▶️ A rejected `disconnect-steps` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_sequence_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "disconnect-steps/rejects-disconnecting-a-missing-edge: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.content.child_id, &base.content.child_id, "a rejected disconnect must not mint a new content handle");
}

/// 🚫️ A missing edge is an Error `mutation.target-missing` addressed at the EDGE id. Unlike
/// `connect-steps`, this verb has exactly one guard: no duplicate check, no invariant check, and the
/// steps collection is never consulted at all.
#[semio_framework_async_macros::async_test]
async fn a_missing_edge_is_an_error_target_missing() {
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &SequenceDiff::default(), "a rejecting disconnect-steps must carry the identity diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing edge is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing disconnect target is an Error, never Fatal");
    assert_eq!(messages[0].target, vec!["edge-gone".to_string()], "the diagnostic addresses the edge id");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[semio_framework_async_macros::async_test]
async fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️.absent must be an empty marker, not a stand-in patch");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &SequenceDiff::default(), "disconnect-steps/rejects-disconnecting-a-missing-edge: a rejection must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical — the payload is a
/// single `id`, with no endpoints: an edge is removed by its own identity, never by its shape.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SequenceSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "disconnect-steps/rejects-disconnecting-a-missing-edge: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "disconnect-steps/rejects-disconnecting-a-missing-edge: committed mutation JSON is not canonical");
    assert!(original.get("from").is_none() && original.get("to").is_none(), "disconnect-steps removes an edge by id alone, never by endpoint pair");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "disconnect-steps/rejects-disconnecting-a-missing-edge declares a rejected outcome");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ↩️ `disconnect-steps`' inverse is BASE-derived — it needs the captured `(id, from, to)` triple to
/// rebuild the edge — so a missing edge yields NO undo step. This is the exact mirror of its partner
/// `connect-steps`, whose payload-derived inverse is produced unconditionally.
#[semio_framework_async_macros::async_test]
async fn inverse_of_a_missing_disconnect_is_empty() {
    let inverse = inverse_sequence_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "disconnect-steps cannot rebuild an edge it never saw, got {inverse:?}");
}

/// 🪪️ The fixture is bound to `disconnect-steps`' own descriptor and its edge-id address.
#[semio_framework_async_macros::async_test]
async fn semantics_bind_this_fixture_to_disconnect_steps() {
    let semantics = <SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("disconnect", "steps", "disconnect-steps", "DisconnectedSteps"), "the fixture must be bound to disconnect-steps' own descriptor");
    assert_eq!(<SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::target(&mutation()), vec!["edge-gone".to_string()], "disconnect-steps addresses the edge id");
}
