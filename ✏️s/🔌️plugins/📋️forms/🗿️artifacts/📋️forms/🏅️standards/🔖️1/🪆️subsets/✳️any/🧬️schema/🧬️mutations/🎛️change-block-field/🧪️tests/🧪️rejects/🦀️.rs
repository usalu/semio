//! 🧪️ `change-block-field` fixture — `🧪️rejects-a-field-of-a-question-the-scene-does-not-hold`: a question no step holds is an Error-level
//! `mutation.target-missing` naming the question id, the document untouched (contract D6: `🔺️diff/🚫️.absent`, `➡️after == ⬅️before`).

use crate::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};
use crate::{replace_forms_steps, FormStep, FormsDiff, FormsSnapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️change-block-field/🧪️rejects/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️change-block-field/🧪️rejects/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️change-block-field/🧪️rejects/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️change-block-field/🧪️rejects/🎯️outcome/🔣️.json");

fn mutation() -> FormMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn expected_after() -> FormsSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, its composed children resolved to the scene its own `definition.steps` lists.
fn before() -> FormsSnapshot {
    let mut snapshot: FormsSnapshot = semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes");
    let document: serde_json::Value = serde_json::from_str(BEFORE).expect("before reparses");
    let steps: Vec<FormStep> = semio_framework_pack_json::from_json_str(&document["definition"]["steps"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed scene decodes");
    replace_forms_steps(&mut snapshot, steps);
    snapshot
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER), ("mutation", MUTATION)] {
        let reencoded = if label == "mutation" { semio_framework_pack_json::to_json_string(&mutation()) } else { semio_framework_pack_json::to_json_string(&semio_framework_pack_json::from_json_str::<FormsSnapshot>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes")) };
        let reencoded: serde_json::Value = serde_json::from_str(&reencoded).expect("re-encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("reparses"), "change-block-field/rejects-a-field-of-a-question-the-scene-does-not-hold: committed {label} JSON is not canonical");
    }
}

/// ▶️ The refused or redundant edit leaves the document at exactly the committed `after`, minting neither composed handle.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_form_edit_mutation(&base, &mutation()).expect("an identity diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "change-block-field/rejects-a-field-of-a-question-the-scene-does-not-hold: applied state differs from committed after-snapshot");
    assert_eq!((&snapshot.structure.child_id, &snapshot.results.child_id), (&base.structure.child_id, &base.results.child_id), "an unapplied edit must not re-mint the structure/results handles");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &FormsDiff::default(), "an unapplied change-block-field carries the identity diff");
}

/// 🎯️ The declared outcome — status, code and path — is exactly what the diff builder emits, at the declared level.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    let declared = outcome.get("code").or_else(|| outcome["messages"][0].get("code")).and_then(serde_json::Value::as_str);
    assert_eq!(declared, Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!((messages[0].code.0.as_str(), messages[0].level), ("mutation.target-missing", semio_framework_diagnostic::Severity::Error));
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).map(|path| path.iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect()).unwrap_or_default();
    assert_eq!(declared_path, messages[0].target, "the declared path must match the emitted target");
    let semantics = <FormMutation as protocol::SemanticMutation<FormsSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "block-field", "change-block-field", "ChangedBlockField"));
}

/// ↩️ The inverse is BASE-derived: applying it after the forward lands on the committed `before` again; a question the base does not hold has nothing to restore.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse_form_mutation(&base, &mutation()).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 0, "change-block-field/rejects-a-field-of-a-question-the-scene-does-not-hold: {inverse:?}");
    let mut snapshot = apply_form_edit_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_form_edit_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-block-field/rejects-a-field-of-a-question-the-scene-does-not-hold: inverse did not restore the before-snapshot");
}
