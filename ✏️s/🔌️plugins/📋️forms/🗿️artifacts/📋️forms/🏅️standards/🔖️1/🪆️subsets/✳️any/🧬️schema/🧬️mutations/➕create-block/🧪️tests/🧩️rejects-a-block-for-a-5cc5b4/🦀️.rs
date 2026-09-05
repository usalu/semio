//! 🧪️ `create-block` fixture — `🧩️rejects-a-block-for-a-step-that-does-not-exist`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `FormsSnapshot` keeps its `steps` tree in the composed
//! `s.stdio.semio.value`/`table` CHILD pair (`🔖️WorkingScene`), so this committed snapshot decodes
//! to an UNRESOLVED handle and `forms_steps` fails soft to an empty scene — no step, therefore no
//! container for a block. A successful `create-block` would route through `forms_diff_from_delta`
//! and re-mint both handles with a `child_id` that is a `DefaultHasher` digest of the child
//! content; forging a value out of `std`'s deliberately unspecified default hasher is not
//! authorable, and this branch reaches no hash at all.
//!
//! ➕ `create-block` is the ONLY verb in this vocabulary that answers a missing CONTAINER with
//! Fatal `mutation.invariant`. Its three siblings that address a block (`delete-block`,
//! `replace-block`, `move-block-to-step`) report the very same missing step as an Error-level
//! `mutation.target-missing`; and this verb's own second guard, a colliding block id, is Fatal
//! `mutation.duplicate-id` with a TWO-segment address. This case pins the first of those three.

use crate::artifacts::forms::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> FormsSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> FormsSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> FormMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A rejected `create-block` leaves the document at exactly the committed `after` and mints
/// neither composed handle.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_form_edit_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-block/rejects-a-block-for-a-step-that-does-not-exist: applied state differs from committed after-snapshot");
    assert_eq!((&mut snapshot.structure, &snapshot.results.child_id), (&base.structure.child_id, &base.results.child_id), "a rejected create must not mint new structure/results handles");
}

/// 🚨️ A missing owning step is FATAL `mutation.invariant` — `create-block`'s signature answer, and
/// the only place this code appears in the forms vocabulary. The address is ONE segment, the step
/// id: the block id joins it only on this verb's other Fatal branch, a duplicate block id.
#[semio_framework_async_macros::async_test]
async fn a_missing_owning_step_is_a_fatal_invariant() {
    let base = before();
    assert!(forms_steps(&base).is_empty(), "rejects-a-block-for-a-step-that-does-not-exist's before-snapshot must decode to an unresolved, stepless scene");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &FormsDiff::default(), "a rejecting create-block must carry an empty diff, never a half-built pair of child handles");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.invariant", "a block whose owning step does not exist breaks an invariant — it is not a plain target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "mutation.invariant is Fatal — no merge policy may absorb it");
    assert_eq!(messages[0].target, vec!["step-outro".to_string()], "the diagnostic names the missing STEP, never the block that could not be placed");
    let semantics = <FormMutation as protocol::SemanticMutation<FormsSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "block", "create-block", "CreatedBlock"), "the fixture must be bound to create-block's own descriptor");
}

/// ↩️ `create-block`'s inverse is BASE-derived, never payload-derived: with no owning step in BASE
/// there is nothing that could have been created, so the inverse is empty rather than a
/// `delete-block` of the requested id.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_block_to_delete() {
    let inverse = inverse_form_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "create-block/rejects-a-block-for-a-step-that-does-not-exist: a refused create must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. Every optional
/// `FormQuestion` field carries `skip_serializing_if = "Option::is_none"`, so the committed block
/// writes only `id`/`label`/`kind`; `CreateBlock::index` carries no skip, so it is an explicit
/// `null`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FormsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-block/rejects-a-block-for-a-step-that-does-not-exist: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-block/rejects-a-block-for-a-step-that-does-not-exist: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-block/rejects-a-block-for-a-step-that-does-not-exist declares a rejected outcome");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
