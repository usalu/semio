//! 🧪️ `change-step-description` fixture — `no-ops-when-clearing-an-already-absent-description`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ⚠️ Why this leaf pins the NO-OP branch: `FormsSnapshot` keeps its `steps` tree in the composed
//! `s.stdio.semio.value`/`table` CHILD pair (`🔖️WorkingScene`), and every content-changing forms
//! diff routes through `forms_diff_from_delta`, which re-mints both handles with a `child_id` that
//! is a `DefaultHasher` digest of the child content. Hand-authoring such an `➡️after` would mean
//! forging a value out of `std`'s deliberately unspecified default hasher. This verb's guard
//! returns before that call, mints nothing, and leaves `➡️after == ⬅️before`.
//!
//! 📝 The interesting branch: `new_description: null` against a step that already has NO
//! description. `ChangeStepDescription` is the one forms verb whose payload field is itself an
//! `Option` used as a CLEAR instruction, so its `existing.description == payload.new_description`
//! guard is the only one in this vocabulary that can fire on a `None == None` comparison. The
//! seeded step takes its `id` and its `description` from the committed payload; its `title` is this
//! case's own fixture value and is deliberately arbitrary — `change-step-description` never reads it.

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

/// 🌱 The committed `⬅️before`, with its composed children resolved to a scene holding one step
/// whose id and (absent) description are the committed payload's own.
fn before() -> FormsSnapshot {
    let snapshot: FormsSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let FormMutation::ChangeStepDescription(payload) = mutation() else {
        panic!("no-ops-when-clearing-an-already-absent-description's committed mutation must be a change-step-description");
    };
    cache_forms_steps(&snapshot.structure.child_id, vec![FormStep { id: payload.id.clone(), title: "Basics".into(), description: payload.new_description.clone(), blocks: Vec::new() }]);
    snapshot
}

/// ▶️ Clearing an already-absent description carries `before` to exactly the committed `after`, and
/// re-mints neither composed handle.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_form_edit_mutation(&base, &mutation()).expect("an identity diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "change-step-description/no-ops-when-clearing-an-already-absent-description: applied state differs from committed after-snapshot");
    assert_eq!((&snapshot.structure.child_id, &snapshot.results.child_id), (&base.structure.child_id, &base.results.child_id), "a refused description change must not re-mint the structure/results handles");
    assert!(forms_steps(&snapshot).first().expect("the seeded scene holds the addressed step").description.is_none(), "the step still has no description at all");
}

/// ↩️ `change-step-description`'s inverse is BASE-derived: it re-reads the step's description out of
/// the working scene, so undoing this refused clear is itself another clear — and another no-op.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_form_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "change-step-description undoes with exactly one step whenever the id resolves, got {inverse:?}");
    let FormMutation::ChangeStepDescription(undo) = &inverse[0] else {
        panic!("change-step-description's inverse must be another change-step-description, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "step-basics", "the inverse addresses the very step the payload named");
    assert!(undo.new_description.is_none(), "the inverse restores the BASE-state description, which this step never had");
    let mut snapshot = apply_form_edit_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = apply_form_edit_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-step-description/no-ops-when-clearing-an-already-absent-description: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. `new_description`
/// carries no `skip_serializing_if`, so the committed payload writes it out as an explicit `null`
/// — that explicitness IS the clear instruction this case is about.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FormsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-step-description/no-ops-when-clearing-an-already-absent-description: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-step-description/no-ops-when-clearing-an-already-absent-description: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-step-description/no-ops-when-clearing-an-already-absent-description declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].code.0, "mutation.no-op", "clearing an absent description is a no-op, never a target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a redundant clear must not escalate to Error or Fatal");
    let semantics = <FormMutation as protocol::SemanticMutation<FormsSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "step-description", "change-step-description", "ChangedStepDescription"), "the fixture must be bound to change-step-description's own descriptor");
}

/// 🔺️ The delta a redundant clear produces is exactly the committed all-null `FormsDiff`: the guard
/// returns before any `FormsStepPatch { description, .. }` is built, so the step is never patched.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-step-description/no-ops-when-clearing-an-already-absent-description: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(outcome.diff(), &FormsDiff::default(), "a refused change-step-description must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to forms' own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-step-description/no-ops-when-clearing-an-already-absent-description: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed identity diff directly to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <FormsDiff as protocol::MutationDiff<FormsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-step-description/no-ops-when-clearing-an-already-absent-description: committed diff did not carry before to after");
}
