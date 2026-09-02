//! 🧪️ `replace-block` fixture — `no-ops-when-the-replacement-block-is-identical`.
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
//! 🔁 `replace-block` is the vocabulary's whole-value swap: `FormQuestion` carries 15+ optional
//! config fields plus a boxed recursive `condition` expression tree, which is exactly why this verb
//! exists instead of a per-field `change-block-*` fan-out — and why its no-op guard is the only one
//! in this vocabulary that is a full structural `PartialEq` over an entire payload struct rather
//! than a single scalar comparison. The seeded step's one block IS the committed payload's block,
//! byte for byte; only the step's `title` is this case's own fixture value.

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

/// 🌱 The committed `⬅️before`, with its composed children resolved to a scene whose single step
/// holds exactly the block the committed payload asks to install — the structural identity the
/// guard tests.
fn before() -> FormsSnapshot {
    let mut snapshot: FormsSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let FormMutation::ReplaceBlock(payload) = mutation() else {
        panic!("no-ops-when-the-replacement-block-is-identical's committed mutation must be a replace-block");
    };
    materialize_forms_steps(&mut snapshot.structure, vec![FormStep { id: payload.step_id.clone(), title: "Basics".into(), description: None, blocks: vec![payload.block.clone()] }]);
    snapshot
}

/// ▶️ Replacing a block with a structurally identical one carries `before` to exactly the committed
/// `after` and re-mints neither composed handle.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_form_edit_mutation(&base, &mutation()).expect("an identity diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "replace-block/no-ops-when-the-replacement-block-is-identical: applied state differs from committed after-snapshot");
    assert_eq!((&mut snapshot.structure, &snapshot.results.child_id), (&base.structure.child_id, &base.results.child_id), "a refused replace must not re-mint the structure/results handles");
    assert_eq!(forms_steps(&snapshot).first().map(|step| step.blocks.len()), Some(1), "the step still holds exactly the one block it started with");
}

/// ↩️ `replace-block`'s inverse is BASE-derived: it hands back the block the scene currently holds,
/// so undoing a refused replace re-offers the very same value — another no-op.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_form_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "replace-block undoes with exactly one step whenever step and block resolve, got {inverse:?}");
    let FormMutation::ReplaceBlock(undo) = &inverse[0] else {
        panic!("replace-block's inverse must be another replace-block, got {:?}", inverse[0]);
    };
    assert_eq!(undo.step_id, "step-basics", "the inverse addresses the same owning step");
    assert_eq!(undo.block.id, "q-site-name", "the inverse restores the BASE-state block under its own id");
    let mut snapshot = apply_form_edit_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = apply_form_edit_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-block/no-ops-when-the-replacement-block-is-identical: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. Every optional
/// `FormQuestion` field carries `skip_serializing_if = "Option::is_none"`, so the committed block
/// legitimately writes only `id`/`label`/`kind`/`required` — and any invented `null` field there
/// would break both this assertion and the structural guard the case depends on.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FormsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-block/no-ops-when-the-replacement-block-is-identical: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-block/no-ops-when-the-replacement-block-is-identical: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op` — the
/// same verb answers a missing step or a missing block with a two-segment Error instead.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-block/no-ops-when-the-replacement-block-is-identical declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].code.0, "mutation.no-op", "an identical replacement is a no-op, never a target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a redundant replace must not escalate to Error or Fatal");
    let semantics = <FormMutation as protocol::SemanticMutation<FormsSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("replace", "block", "replace-block", "ReplacedBlock"), "the fixture must be bound to replace-block's own descriptor");
}

/// 🔺️ The delta an identical replacement produces is exactly the committed all-null `FormsDiff`:
/// the guard returns before the step's `blocks` Vec is cloned, so no `FormsStepPatch` is built.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-block/no-ops-when-the-replacement-block-is-identical: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(outcome.diff(), &FormsDiff::default(), "a refused replace-block must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to forms' own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-block/no-ops-when-the-replacement-block-is-identical: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed identity diff directly to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <FormsDiff as protocol::MutationDiff<FormsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-block/no-ops-when-the-replacement-block-is-identical: committed diff did not carry before to after");
}
