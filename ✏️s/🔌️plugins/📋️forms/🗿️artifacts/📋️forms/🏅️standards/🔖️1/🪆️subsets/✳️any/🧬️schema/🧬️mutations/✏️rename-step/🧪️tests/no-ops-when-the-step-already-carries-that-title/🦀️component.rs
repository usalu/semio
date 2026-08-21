//! 🧪️ `rename-step` fixture — `no-ops-when-the-step-already-carries-that-title`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ⚠️ Why this leaf pins the NO-OP branch: `FormsSnapshot` keeps its whole `steps` tree in the
//! composed `s.stdio.semio.value`/`table` CHILD pair (`🔖️WorkingScene`), so a committed snapshot
//! carries two handles and never a step. Every content-changing forms diff routes through
//! `forms_diff_from_delta`, which re-mints both handles with a `child_id` that is a
//! `DefaultHasher` digest of the child content — hand-authoring such an `➡️after` would mean
//! forging a value from `std`'s deliberately unspecified default hasher. `rename-step`'s guard
//! returns BEFORE that call, so it mints nothing and `➡️after == ⬅️before`; the case is applied
//! with the identity diff and one Warning, exactly as `MutationOutcome::empty().warn(..)` builds it.
//!
//! ✏️ Nothing in the seeded scene is invented: the one step it holds takes its `id` AND its `title`
//! straight from the committed payload — that identity is precisely the collision the guard tests.

use crate::artifacts::forms::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};
use crate::artifacts::forms::{cache_forms_steps, forms_steps, FormStep, FormsDiff, FormsSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> FormMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> FormsSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed children resolved to a scene holding exactly one
/// step whose id and title are the committed payload's own `id`/`new_title`.
fn before() -> FormsSnapshot {
    let snapshot: FormsSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let FormMutation::RenameStep(payload) = mutation() else {
        panic!("no-ops-when-the-step-already-carries-that-title's committed mutation must be a rename-step");
    };
    cache_forms_steps(&snapshot.structure.child_id, vec![FormStep { id: payload.id.clone(), title: payload.new_title.clone(), description: None, blocks: Vec::new() }]);
    snapshot
}

/// ▶️ A refused rename carries `before` to exactly the committed `after`, and — critically — leaves
/// both composed handles alone: a no-op must never re-mint content.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_form_edit_mutation(&base, &mutation()).expect("an identity diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "rename-step/no-ops-when-the-step-already-carries-that-title: applied state differs from committed after-snapshot");
    assert_eq!((&snapshot.structure.child_id, &snapshot.results.child_id), (&base.structure.child_id, &base.results.child_id), "a refused rename must not re-mint the structure/results handles");
    assert_eq!(forms_steps(&snapshot).first().map(|step| step.title.clone()), Some("Basics".to_string()), "the step keeps the title it already carried");
}

/// ↩️ `rename-step`'s inverse is BASE-derived, never payload-derived: it re-reads the step's title
/// out of the working scene, so undoing a refused rename is itself another no-op on the same title.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_form_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "rename-step undoes with exactly one step whenever the id resolves, got {inverse:?}");
    let FormMutation::RenameStep(undo) = &inverse[0] else {
        panic!("rename-step's inverse must be another rename-step, got {:?}", inverse[0]);
    };
    assert_eq!((undo.id.as_str(), undo.new_title.as_str()), ("step-basics", "Basics"), "the inverse restores the BASE-state title of the very step the payload addressed");
    let mut snapshot = apply_form_edit_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = apply_form_edit_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-step/no-ops-when-the-step-already-carries-that-title: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. `RenameStep`
/// carries NO `rename_all` of its own, so its fields stay snake_case on the wire (`new_title`) even
/// though the enum tag itself is camelCased to `renameStep`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FormsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-step/no-ops-when-the-step-already-carries-that-title: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-step/no-ops-when-the-step-already-carries-that-title: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds: `applied`, carrying one Warning `mutation.no-op`. The warning is
/// deliberately untargeted — `MutationOutcome::warn` takes no address, unlike the Error-level
/// `mutation.target-missing` this same verb raises when the id does not resolve.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-step/no-ops-when-the-step-already-carries-that-title declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].code.0, "mutation.no-op", "an already-current title is a no-op, not a target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "renaming a step to its current title must not escalate to Error or Fatal");
    assert!(messages[0].target.is_empty(), "rename-step's no-op warning carries no target address");
    let semantics = <FormMutation as protocol::SemanticMutation<FormsSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("rename", "step", "rename-step", "RenamedStep"), "the fixture must be bound to rename-step's own descriptor");
}

/// 🔺️ The delta a refused rename produces is exactly the committed all-null `FormsDiff` — the guard
/// returns before `forms_diff_from_delta` is ever reached, so no half-built handle escapes.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-step/no-ops-when-the-step-already-carries-that-title: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(outcome.diff(), &FormsDiff::default(), "a refused rename-step must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to forms' own diff type: `FormsDiff` has
/// `#[serde(rename_all = "camelCase", default)]` and no per-field `skip_serializing_if`, so all
/// eleven fields are written out as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-step/no-ops-when-the-step-already-carries-that-title: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed identity diff directly to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <FormsDiff as protocol::MutationDiff<FormsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-step/no-ops-when-the-step-already-carries-that-title: committed diff did not carry before to after");
}
