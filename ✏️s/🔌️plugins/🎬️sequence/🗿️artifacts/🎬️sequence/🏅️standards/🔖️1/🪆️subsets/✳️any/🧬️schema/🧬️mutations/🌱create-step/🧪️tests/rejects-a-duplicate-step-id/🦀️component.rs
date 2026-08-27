//! 🧪️ `create-step` fixture — `rejects-a-duplicate-step-id`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why every sequence case pins a GUARD branch: `SequenceSnapshot` keeps its steps and edges in
//! the composed `s.stdio.semio.flow` CHILD (`🔖️WorkingScene`), so a committed snapshot carries a
//! handle, never a graph — and every content-changing diff routes through `diff_replace_content`,
//! which mints a fresh handle whose `child_id` is a `DefaultHasher` digest. Hand-authoring such an
//! `➡️after` would mean forging a value from `std`'s deliberately unspecified default hasher, so
//! this tree pins the branches that mint no handle at all.
//!
//! 🌱 `create-step` has exactly one guard and it is FATAL: a colliding step id breaks the id-keyed
//! collection's identity invariant. The seeded scene holds exactly the step the committed payload
//! asks to create.

use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::{apply_sequence_mutation, inverse_sequence_mutation, SequenceMutation};
use crate::artifacts::sequence::{SequenceSnapshot, SequenceWorkingScene};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️component.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> SequenceMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> SequenceSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `content` child resolved to a scene holding
/// exactly the step the committed payload carries, and no edges — the collision the Fatal guards.
fn before() -> SequenceSnapshot {
    let snapshot: SequenceSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let SequenceMutation::CreateStep(payload) = mutation() else {
        panic!("rejects-a-duplicate-step-id's committed mutation must be a create-step");
    };
    snapshot.content.set_local_owner(std::sync::Arc::new(SequenceWorkingScene { steps: vec![payload.step.clone()], edges: Vec::new() }));
    snapshot
}

/// ▶️ A rejected `create-step` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_sequence_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-step/rejects-a-duplicate-step-id: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.content.child_id, &base.content.child_id, "a rejected create must not mint a new content handle");
}

/// 🚨️ A colliding step id is FATAL `mutation.duplicate-id`, addressed at the bare step id —
/// sequence's addresses are single-segment, unlike present's `tiles`-prefixed ones.
#[semio_framework_async_macros::async_test]
async fn a_colliding_step_id_is_fatal() {
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &SequenceDiff::default(), "a rejecting create-step must carry the identity diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "an id collision is reported as duplicate-id");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "duplicate-id is Fatal — no merge policy may absorb it");
    assert_eq!(messages[0].target, vec!["step-log".to_string()], "the diagnostic addresses the colliding step id");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[semio_framework_async_macros::async_test]
async fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️component.absent must be an empty marker, not a stand-in patch");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &SequenceDiff::default(), "create-step/rejects-a-duplicate-step-id: a Fatal outcome must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. `SequenceStep`'s
/// fields are `#[serde(default)]` but carry no skip attribute, so the payload's step serializes ALL
/// seven fields — `slot` explicitly `null`, never omitted.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SequenceSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-step/rejects-a-duplicate-step-id: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-step/rejects-a-duplicate-step-id: committed mutation JSON is not canonical");
    assert!(original.get("step").and_then(|step| step.get("slot")).map(serde_json::Value::is_null).unwrap_or(false), "an unslotted step serializes slot as an explicit null");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-step/rejects-a-duplicate-step-id declares a rejected outcome");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ↩️ `create-step`'s inverse is PAYLOAD-derived — a `delete-step` of the id it was asked to create,
/// produced even here where the create was refused as a duplicate.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_delete_of_the_requested_id_even_when_refused() {
    let inverse = inverse_sequence_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "create-step always undoes with exactly one step, got {inverse:?}");
    let SequenceMutation::DeleteStep(undo) = &inverse[0] else {
        panic!("create-step's inverse must be a delete-step, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "step-log", "the inverse deletes exactly the id the payload carried");
}

/// 🪪️ The fixture is bound to `create-step`'s own descriptor and its single-segment address.
#[semio_framework_async_macros::async_test]
async fn semantics_bind_this_fixture_to_create_step() {
    let semantics = <SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "step", "create-step", "CreatedStep"), "the fixture must be bound to create-step's own descriptor");
    assert_eq!(<SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::target(&mutation()), vec!["step-log".to_string()], "create-step addresses the new step id");
}
