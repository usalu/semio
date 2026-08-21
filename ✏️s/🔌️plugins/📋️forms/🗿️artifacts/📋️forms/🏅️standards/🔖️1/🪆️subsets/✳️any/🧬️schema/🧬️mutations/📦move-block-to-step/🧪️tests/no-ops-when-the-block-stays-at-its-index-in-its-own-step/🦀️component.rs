//! 🧪️ `move-block-to-step` fixture — `no-ops-when-the-block-stays-at-its-index-in-its-own-step`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ⚠️ Why this leaf pins the NO-OP branch: `FormsSnapshot` keeps its `steps` tree in the composed
//! `s.stdio.semio.value`/`table` CHILD pair (`🔖️WorkingScene`), and every content-changing forms
//! diff routes through `forms_diff_from_delta`, which re-mints both handles with a `child_id` that
//! is a `DefaultHasher` digest of the child content — hand-authoring such an `➡️after` would mean
//! forging a value out of `std`'s deliberately unspecified default hasher. This guard returns
//! before that call, so nothing is minted and `➡️after == ⬅️before`.
//!
//! 📦 `move-block-to-step` is the vocabulary's only container-crossing verb (derivation-rules rule
//! 5's `move-to-<container>`), and it carries a branch no sibling has: when `step_id == to_step_id`
//! it degenerates into an in-list reorder and short-circuits BEFORE the destination step is ever
//! looked up. That same-step branch is the one this case pins — with a two-block step, so that the
//! `index.min(blocks.len())` clamp is computed against a list the block has already been lifted out
//! of, exactly as it is for a genuine move.

use crate::artifacts::forms::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};
use crate::artifacts::forms::{cache_forms_steps, forms_steps, FormQuestion, FormStep, FormsDiff, FormsSnapshot};

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
fn block(id: &str, label: &str) -> FormQuestion {
    FormQuestion {
        id: id.into(),
        label: label.into(),
        kind: "text".into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}

/// 🌱 The committed `⬅️before`, with its composed children resolved to a single step holding two
/// blocks — the committed payload's own `q-site-name` first, at the index it asks to move to.
fn before() -> FormsSnapshot {
    let snapshot: FormsSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let FormMutation::MoveBlockToStep(payload) = mutation() else {
        panic!("no-ops-when-the-block-stays-at-its-index-in-its-own-step's committed mutation must be a move-block-to-step");
    };
    cache_forms_steps(&snapshot.structure.child_id, vec![FormStep { id: payload.step_id.clone(), title: "Basics".into(), description: None, blocks: vec![block(&payload.block_id, "Site name"), block("q-visit-date", "Visit date")] }]);
    snapshot
}

/// ▶️ Moving a block onto the index it already holds within its own step carries `before` to
/// exactly the committed `after`, leaves the block order alone, and re-mints neither handle.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_form_edit_mutation(&base, &mutation()).expect("an identity diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "move-block-to-step/no-ops-when-the-block-stays-at-its-index-in-its-own-step: applied state differs from committed after-snapshot");
    assert_eq!((&snapshot.structure.child_id, &snapshot.results.child_id), (&base.structure.child_id, &base.results.child_id), "a refused move must not re-mint the structure/results handles");
    assert_eq!(forms_steps(&snapshot).first().map(|step| step.blocks.iter().map(|block| block.id.clone()).collect::<Vec<_>>()), Some(vec!["q-site-name".to_string(), "q-visit-date".to_string()]), "the step keeps its original block order");
}

/// ↩️ `move-block-to-step`'s inverse is the MIRROR of the payload: source and destination steps are
/// swapped and the index is read back out of BASE. For this same-step move the mirror is the
/// payload itself, so undoing it is another no-op.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_form_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "move-block-to-step undoes with exactly one step whenever the source block resolves, got {inverse:?}");
    let FormMutation::MoveBlockToStep(undo) = &inverse[0] else {
        panic!("move-block-to-step's inverse must be another move-block-to-step, got {:?}", inverse[0]);
    };
    assert_eq!((undo.step_id.as_str(), undo.to_step_id.as_str()), ("step-basics", "step-basics"), "the inverse swaps source and destination — identical here because this move never leaves its step");
    assert_eq!((undo.block_id.as_str(), undo.index), ("q-site-name", 0), "the inverse sends the block back to the index it held in BASE");
    let mut snapshot = apply_form_edit_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = apply_form_edit_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "move-block-to-step/no-ops-when-the-block-stays-at-its-index-in-its-own-step: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. `MoveBlockToStep`
/// declares no `rename_all` of its own, so `step_id`/`block_id`/`to_step_id` stay snake_case on the
/// wire while the enum tag is camelCased to `moveBlockToStep`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FormsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-block-to-step/no-ops-when-the-block-stays-at-its-index-in-its-own-step: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-block-to-step/no-ops-when-the-block-stays-at-its-index-in-its-own-step: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op`. This
/// verb reaches the warning only through its same-step branch; a cross-step move has no no-op guard
/// at all.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-block-to-step/no-ops-when-the-block-stays-at-its-index-in-its-own-step declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].code.0, "mutation.no-op", "a block already at the requested index is a no-op, never a target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a redundant in-step move must not escalate to Error or Fatal");
    let semantics = <FormMutation as protocol::SemanticMutation<FormsSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("move", "block", "move-block-to-step", "MovedBlockToStep"),
        "the fixture must be bound to move-block-to-step's own descriptor — `move` because it crosses a container boundary, unlike a plain `reorder`"
    );
}

/// 🔺️ The delta a redundant in-step move produces is exactly the committed all-null `FormsDiff`:
/// the guard returns before the single-step `FormsStepPatch { blocks, .. }` is assembled, so the
/// two-entry cross-step shape this verb builds otherwise is never reached either.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-block-to-step/no-ops-when-the-block-stays-at-its-index-in-its-own-step: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(outcome.diff(), &FormsDiff::default(), "a refused move-block-to-step must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to forms' own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-block-to-step/no-ops-when-the-block-stays-at-its-index-in-its-own-step: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed identity diff directly to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <FormsDiff as protocol::MutationDiff<FormsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-block-to-step/no-ops-when-the-block-stays-at-its-index-in-its-own-step: committed diff did not carry before to after");
}
