//! 🧪️ `delete-step` fixture — `rejects-deleting-a-step-the-scene-does-not-hold`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `FormsSnapshot` keeps its `steps` tree in the composed
//! `s.stdio.semio.value`/`table` CHILD pair (`🔖️WorkingScene`), and a successful `delete-step`
//! routes through `forms_diff_from_delta`, which re-mints both handles with a `child_id` that is a
//! `DefaultHasher` digest of the child content — hand-authoring such an `➡️after` would mean
//! forging a value out of `std`'s deliberately unspecified default hasher. `delete-step` has no
//! no-op guard, so a rejection is its only branch that mints nothing.
//!
//! 🗑️ The scene here is deliberately NOT empty: it holds `step-basics`, while the committed payload
//! asks to delete `step-outro`. That distinguishes a genuine id miss from the trivial "nothing was
//! resolved" case — `delete-step` scans the document's top-level `steps` collection by id, and the
//! diagnostic it returns is that id alone, one segment, with no cascade note about the blocks a
//! real delete would have carried away with it.

use crate::artifacts::forms::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};
use crate::artifacts::forms::{materialize_forms_steps, forms_steps, FormStep, FormsDiff, FormsSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> FormMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> FormsSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed children resolved to a one-step scene that holds
/// `step-basics` — every step EXCEPT the one the committed payload addresses.
fn before() -> FormsSnapshot {
    let mut snapshot: FormsSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    materialize_forms_steps(&mut snapshot.structure, vec![FormStep { id: "step-basics".into(), title: "Basics".into(), description: None, blocks: Vec::new() }]);
    snapshot
}

/// ▶️ A rejected `delete-step` leaves the document at exactly the committed `after` and mints
/// neither composed handle.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_form_edit_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "delete-step/rejects-deleting-a-step-the-scene-does-not-hold: applied state differs from committed after-snapshot");
    assert_eq!((&mut snapshot.structure, &snapshot.results.child_id), (&base.structure.child_id, &base.results.child_id), "a rejected delete must not mint new structure/results handles");
    assert_eq!(forms_steps(&snapshot).len(), 1, "the step the scene DOES hold must survive a delete aimed at another id");
}

/// 🗑️ A step id the top-level `steps` collection does not carry is an Error-level
/// `mutation.target-missing` addressed by that id alone — one segment, because `delete-step`
/// searches steps, not any nested `blocks` list, and reports no per-block cascade for a delete it
/// never performed.
#[semio_framework_async_macros::async_test]
async fn a_missing_step_is_reported_by_its_step_id_alone() {
    let base = before();
    assert!(forms_steps(&base).iter().all(|step| step.id != "step-outro"), "rejects-deleting-a-step-the-scene-does-not-hold's scene must not contain the addressed step");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &FormsDiff::default(), "a rejecting delete-step must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing step is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "delete-step has no Fatal branch — deleting what is not there is recoverable, unlike create-step's duplicate id");
    assert_eq!(messages[0].target, vec!["step-outro".to_string()], "the diagnostic names the step id and nothing else");
    let semantics = <FormMutation as protocol::SemanticMutation<FormsSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "step", "delete-step", "DeletedStep"), "the fixture must be bound to delete-step's own descriptor");
}

/// ↩️ `delete-step` inverts by reconstructing the step BASE held — its blocks and its BASE index —
/// as a `create-step`. With no such step captured the inverse is empty; it never falls back to a
/// payload-derived undo.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_step_to_recreate() {
    let inverse = inverse_form_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "delete-step/rejects-deleting-a-step-the-scene-does-not-hold: a rejected delete must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FormsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-step/rejects-deleting-a-step-the-scene-does-not-hold: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-step/rejects-deleting-a-step-the-scene-does-not-hold: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "delete-step/rejects-deleting-a-step-the-scene-does-not-hold declares a rejected outcome");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
