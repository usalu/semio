//! 🧪️ `reorder-step` fixture — `🚪️no-ops-when-the-step-already-sits-at-that-index`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ⚠️ Why this leaf pins the NO-OP branch: `FormsSnapshot` keeps its `steps` tree in the composed
//! `s.stdio.semio.value`/`table` CHILD pair (`🔖️WorkingScene`), and every content-changing forms
//! diff routes through `forms_diff_from_delta`, which re-mints both handles with a `child_id` that
//! is a `DefaultHasher` digest of the child content — an `➡️after` for a real reorder would mean
//! forging a value out of `std`'s deliberately unspecified default hasher. The guard below returns
//! before that call, so nothing is minted and `➡️after == ⬅️before`.
//!
//! 🔀 `reorder-step` is the only forms verb whose guard compares a payload INDEX rather than a
//! value: `to_index` is a FINAL-state index, so the builder first drops the step from the order
//! list and only then clamps and compares against the step's original position. Pinning index 1 of
//! a three-step scene (rather than a degenerate single-step one) is what makes that ordering
//! semantics observable; the two sibling steps are this case's own fixture scene, the addressed
//! `step-photos` is the committed payload's own id.

use crate::artifacts::forms::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};
use crate::artifacts::forms::{materialize_forms_steps, forms_steps, FormStep, FormsDiff, FormsSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> FormMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> FormsSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn step(id: &str, title: &str) -> FormStep {
    FormStep { id: id.into(), title: title.into(), description: None, blocks: Vec::new() }
}

/// 🌱 The committed `⬅️before`, with its composed children resolved to the three-step survey this
/// case reorders within — `step-photos`, the committed payload's own id, sits at index 1.
fn before() -> FormsSnapshot {
    let mut snapshot: FormsSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    materialize_forms_steps(&mut snapshot.structure, vec![step("step-basics", "Basics"), step("step-photos", "Photos"), step("step-summary", "Summary")]);
    snapshot
}

/// ▶️ Reordering a step onto the index it already occupies carries `before` to exactly the
/// committed `after`, leaves the step order untouched, and re-mints neither composed handle.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_form_edit_mutation(&base, &mutation()).expect("an identity diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "reorder-step/no-ops-when-the-step-already-sits-at-that-index: applied state differs from committed after-snapshot");
    assert_eq!((&mut snapshot.structure, &snapshot.results.child_id), (&base.structure.child_id, &base.results.child_id), "a refused reorder must not re-mint the structure/results handles");
    assert_eq!(forms_steps(&snapshot).iter().map(|step| step.id.clone()).collect::<Vec<_>>(), vec!["step-basics".to_string(), "step-photos".to_string(), "step-summary".to_string()], "the survey keeps its original step order");
}

/// ↩️ `reorder-step`'s inverse is BASE-derived: it reads the step's CURRENT position out of the
/// working scene and targets that, so undoing a refused reorder is another move to index 1.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_form_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "reorder-step undoes with exactly one step whenever the id resolves, got {inverse:?}");
    let FormMutation::ReorderStep(undo) = &inverse[0] else {
        panic!("reorder-step's inverse must be another reorder-step, got {:?}", inverse[0]);
    };
    assert_eq!((undo.id.as_str(), undo.to_index), ("step-photos", 1), "the inverse sends the step back to the index it held in BASE");
    let mut snapshot = apply_form_edit_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = apply_form_edit_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "reorder-step/no-ops-when-the-step-already-sits-at-that-index: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. `ReorderStep`
/// declares no `rename_all` of its own, so `to_index` stays snake_case on the wire while the enum
/// tag is camelCased to `reorderStep`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FormsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-step/no-ops-when-the-step-already-sits-at-that-index: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-step/no-ops-when-the-step-already-sits-at-that-index: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op` — the
/// same verb answers an unknown id with an Error-level `mutation.target-missing` instead.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-step/no-ops-when-the-step-already-sits-at-that-index declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].code.0, "mutation.no-op", "a step already at the requested index is a no-op, never a target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a redundant reorder must not escalate to Error or Fatal");
    let semantics = <FormMutation as protocol::SemanticMutation<FormsSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("reorder", "step", "reorder-step", "ReorderedStep"),
        "the fixture must be bound to reorder-step's own descriptor — reorder is list repositioning, never the spatial `move` verb"
    );
}

/// 🔺️ The delta a redundant reorder produces is exactly the committed all-null `FormsDiff`: the
/// guard returns before any `FormsStepsDelta { reordered, .. }` reaches `forms_diff_from_delta`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-step/no-ops-when-the-step-already-sits-at-that-index: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(outcome.diff(), &FormsDiff::default(), "a refused reorder-step must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to forms' own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-step/no-ops-when-the-step-already-sits-at-that-index: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed identity diff directly to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <FormsDiff as protocol::MutationDiff<FormsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-step/no-ops-when-the-step-already-sits-at-that-index: committed diff did not carry before to after");
}
