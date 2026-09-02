//! 🧪️ `create-step` fixture — `rejects-a-duplicate-step-id`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `FormsSnapshot` keeps its `steps` tree in the composed
//! `s.stdio.semio.value`/`table` CHILD pair (`🔖️WorkingScene`), so a committed snapshot carries two
//! handles and never a step. A successful `create-step` routes through `forms_diff_from_delta`,
//! which re-mints both handles with a `child_id` that is a `DefaultHasher` digest of the child
//! content — hand-authoring such an `➡️after` would mean forging a value out of `std`'s
//! deliberately unspecified default hasher. The Fatal branch below reaches no hash at all.
//!
//! 🌱 `create-step` is the only forms verb with NO no-op guard and no `target-missing` branch: an
//! empty scene simply accepts it. So this case seeds the working scene with exactly the step the
//! committed payload asks to create — nothing here is invented, the seeded step IS the mutation
//! JSON's own `step` — which is precisely the collision `mutation.duplicate-id` guards against.

use crate::artifacts::forms::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};
use crate::artifacts::forms::{materialize_forms_steps, FormsDiff, FormsSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> FormMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> FormsSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed children resolved to a scene holding exactly the
/// step the committed payload tries to create.
fn before() -> FormsSnapshot {
    let mut snapshot: FormsSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let FormMutation::CreateStep(payload) = mutation() else {
        panic!("rejects-a-duplicate-step-id's committed mutation must be a create-step");
    };
    materialize_forms_steps(&mut snapshot.structure, vec![payload.step.clone()]);
    snapshot
}

/// ▶️ A rejected `create-step` leaves the document at exactly the committed `after` — and, above
/// all, does not mint the pair of content-addressed handles a successful create would.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_form_edit_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-step/rejects-a-duplicate-step-id: applied state differs from committed after-snapshot");
    assert_eq!((&mut snapshot.structure, &snapshot.results.child_id), (&base.structure.child_id, &base.results.child_id), "a rejected create must not mint new structure/results handles");
}

/// 🚨️ A colliding step id is FATAL `mutation.duplicate-id`, not the Error-level `target-missing`
/// the delete/rename/reorder/move/replace verbs of this vocabulary raise — re-creating an existing
/// id is an invariant breach, not a miss — and the diagnostic addresses the step id alone, one
/// segment, because `steps` is the document's top-level collection.
#[semio_framework_async_macros::async_test]
async fn a_colliding_step_id_is_a_fatal_duplicate_id() {
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &FormsDiff::default(), "a rejecting create-step must carry an empty diff, never a half-built pair of child handles");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "an id collision is reported as duplicate-id");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "duplicate-id is Fatal — no merge policy may absorb it");
    assert_eq!(messages[0].target, vec!["step-basics".to_string()], "the diagnostic addresses the colliding step id, and nothing else");
    let semantics = <FormMutation as protocol::SemanticMutation<FormsSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "step", "create-step", "CreatedStep"), "the fixture must be bound to create-step's own descriptor");
}

/// ↩️ `create-step`'s inverse is BASE-derived in the one way that matters here: when the id already
/// exists in BASE the create was refused, so there is nothing to undo and the inverse is EMPTY —
/// the opposite of dag's `create-node`, whose inverse deletes the requested id regardless.
#[semio_framework_async_macros::async_test]
async fn inverse_has_nothing_to_undo() {
    let inverse = inverse_form_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "create-step/rejects-a-duplicate-step-id: a refused create must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. `CreateStep`
/// declares no `rename_all` and no `skip_serializing_if`, so its `index` is written as an explicit
/// `null`; `FormStep::description` does carry the skip, so the committed step omits it entirely.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FormsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-step/rejects-a-duplicate-step-id: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-step/rejects-a-duplicate-step-id: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-step/rejects-a-duplicate-step-id declares a rejected outcome");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
