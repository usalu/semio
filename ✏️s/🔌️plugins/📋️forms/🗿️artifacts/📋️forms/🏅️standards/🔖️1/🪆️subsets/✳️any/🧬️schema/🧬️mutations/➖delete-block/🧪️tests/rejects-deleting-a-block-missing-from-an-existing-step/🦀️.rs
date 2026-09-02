//! 🧪️ `delete-block` fixture — `rejects-deleting-a-block-missing-from-an-existing-step`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `FormsSnapshot` keeps its `steps` tree in the composed
//! `s.stdio.semio.value`/`table` CHILD pair (`🔖️WorkingScene`), and a successful `delete-block`
//! routes through `forms_diff_from_delta`, which re-mints both handles with a `child_id` that is a
//! `DefaultHasher` digest of the child content — hand-authoring such an `➡️after` would mean
//! forging a value out of `std`'s deliberately unspecified default hasher. `delete-block` has no
//! no-op guard at all, so a rejection is the only branch that mints nothing.
//!
//! ➖ `delete-block` has TWO `mutation.target-missing` branches, and this case pins the SECOND one
//! — the nested-collection miss. The scene deliberately DOES hold the owning step (built from the
//! committed payload's own `step_id`, with an empty `blocks` list), so the first branch is passed
//! and the diagnostic comes back with a TWO-segment address, step id then block id: the outermost
//! container first, exactly as `MutationMessage::at` specifies.

use crate::artifacts::forms::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};
use crate::artifacts::forms::{materialize_forms_steps, forms_steps, FormStep, FormsDiff, FormsSnapshot};

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

/// 🌱 The committed `⬅️before`, with its composed children resolved to a scene that DOES hold the
/// payload's owning step — carrying no blocks, so only the nested lookup can miss.
fn before() -> FormsSnapshot {
    let mut snapshot: FormsSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let FormMutation::DeleteBlock(payload) = mutation() else {
        panic!("rejects-deleting-a-block-missing-from-an-existing-step's committed mutation must be a delete-block");
    };
    materialize_forms_steps(&mut snapshot.structure, vec![FormStep { id: payload.step_id.clone(), title: "Basics".into(), description: None, blocks: Vec::new() }]);
    snapshot
}

/// ▶️ A rejected `delete-block` leaves the document at exactly the committed `after` and mints
/// neither composed handle.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_form_edit_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "delete-block/rejects-deleting-a-block-missing-from-an-existing-step: applied state differs from committed after-snapshot");
    assert_eq!((&mut snapshot.structure, &snapshot.results.child_id), (&base.structure.child_id, &base.results.child_id), "a rejected delete must not mint new structure/results handles");
}

/// ✂️ The step resolves, the block does not: an Error-level `mutation.target-missing` addressed by
/// BOTH ids. `delete-block` searches `step.blocks`, never the top-level `steps` collection, for the
/// id it was handed — that nested search is what earns the second address segment.
#[semio_framework_async_macros::async_test]
async fn a_missing_block_is_reported_under_its_owning_step() {
    let base = before();
    assert_eq!(forms_steps(&base).len(), 1, "rejects-deleting-a-block-missing-from-an-existing-step's scene must hold the owning step, so the step-level branch is passed");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &FormsDiff::default(), "a rejecting delete-block must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a block that is not in its step is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "delete-block has no Fatal branch at all — a miss is recoverable, never an invariant breach");
    assert_eq!(messages[0].target, vec!["step-basics".to_string(), "q-nickname".to_string()], "the diagnostic addresses step then block, outermost container first");
    let semantics = <FormMutation as protocol::SemanticMutation<FormsSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "block", "delete-block", "DeletedBlock"), "the fixture must be bound to delete-block's own descriptor");
}

/// ↩️ `delete-block` inverts by reconstructing the exact block BASE held, at its BASE index, as a
/// `create-block`. With no such block captured the inverse is empty — this verb never invents a
/// payload-derived undo.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_block_to_recreate() {
    let inverse = inverse_form_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "delete-block/rejects-deleting-a-block-missing-from-an-existing-step: a rejected delete must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. `DeleteBlock`
/// declares no `rename_all` of its own, so `step_id` stays snake_case on the wire while the enum
/// tag is camelCased to `deleteBlock`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FormsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-block/rejects-deleting-a-block-missing-from-an-existing-step: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-block/rejects-deleting-a-block-missing-from-an-existing-step: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and the two-segment path — is exactly what the diff
/// builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "delete-block/rejects-deleting-a-block-missing-from-an-existing-step declares a rejected outcome");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
