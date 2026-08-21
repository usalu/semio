//! 🧪️ `change-form-title` fixture — `titles-an-untitled-survey`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🏷️ `change-form-title` is the ONE forms verb whose whole subject is the document's own inline
//! `title` scalar — it never reads `forms_steps`, never builds a `FormsStepsDelta`, and never calls
//! `forms_diff_from_delta`. That is exactly why this leaf can carry a REAL applied case while the
//! nine step/block verbs cannot: their diffs regenerate the composed `structure`/`results` handles,
//! whose `child_id` is a `DefaultHasher` digest of the child content, and hand-forging a value out
//! of `std`'s deliberately unspecified default hasher is not authorable. Here both handles are
//! carried through untouched, so `➡️after` differs from `⬅️before` in one key only: `title`.

use crate::artifacts::forms::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};
use crate::artifacts::forms::{FormsDiff, FormsSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> FormsSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> FormsSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> FormMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ Titling the survey carries `before` to exactly the committed `after` — and touches nothing
/// but `title`: both composed child handles come through byte-identical, because this verb never
/// regenerates the `structure`/`results` pair.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_form_edit_mutation(&base, &mutation()).expect("change-form-title applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-form-title/titles-an-untitled-survey: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.title.as_deref(), Some("Site Survey"), "change-form-title must set the document title the payload carried");
    assert_eq!((&snapshot.structure.child_id, &snapshot.results.child_id), (&base.structure.child_id, &base.results.child_id), "a title change must never re-mint the structure/results child handles");
}

/// ↩️ `change-form-title`'s inverse is BASE-derived, not payload-derived: it restores whatever the
/// base carried — here `None`, since this survey started untitled — so the round trip clears the
/// title again.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_form_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "change-form-title always undoes with exactly one step, got {inverse:?}");
    let FormMutation::ChangeFormTitle(undo) = &inverse[0] else {
        panic!("change-form-title's inverse must be another change-form-title, got {:?}", inverse[0]);
    };
    assert!(undo.new_title.is_none(), "the inverse must restore the BASE title, which this untitled survey did not have");
    let mut snapshot = apply_form_edit_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = apply_form_edit_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-form-title/titles-an-untitled-survey: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point. `FormsSnapshot::title` carries `skip_serializing_if = "Option::is_none"`, so the
/// untitled `⬅️before` legitimately has NO `title` key at all — writing an explicit `null` there
/// would break this assertion.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FormsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-form-title/titles-an-untitled-survey: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-form-title/titles-an-untitled-survey: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is a clean `applied` — a title that genuinely changes value clears the
/// `payload.new_title == base.title` guard, so no `mutation.no-op` warning is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-form-title/titles-an-untitled-survey declares an applied outcome");
    assert!(outcome.get("messages").is_none(), "change-form-title/titles-an-untitled-survey declares no diagnostics");
    let produced = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "a real title change must raise no diagnostic, got {:?}", produced.messages());
    assert!(apply_form_edit_mutation(&before(), &mutation()).is_ok(), "change-form-title/titles-an-untitled-survey: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff — the load-bearing assertion: `title` is the
/// single non-null field, proving `change-form-title` writes the inline scalar and leaves the
/// composed `structure`/`results` slots alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <FormMutation as protocol::Mutation<FormsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-form-title/titles-an-untitled-survey: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(outcome.diff().title, Some(Some("Site Survey".to_string())), "the diff's double-Option title must be an explicit SET, never a clear");
    assert!(outcome.diff().structure.is_none(), "change-form-title must leave the composed structure slot untouched");
    assert!(outcome.diff().results.is_none(), "change-form-title must leave the composed results slot untouched");
}

/// 🔣️ The committed diff is itself canonical and decodes to forms' own diff type. `FormsDiff`
/// carries `#[serde(rename_all = "camelCase", default)]` with no per-field `skip_serializing_if`,
/// so every one of its eleven fields is emitted — `null` for the ten this verb never writes.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-form-title/titles-an-untitled-survey: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the title change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FormsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <FormsDiff as protocol::MutationDiff<FormsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-form-title/titles-an-untitled-survey: committed diff did not carry before to after");
}
