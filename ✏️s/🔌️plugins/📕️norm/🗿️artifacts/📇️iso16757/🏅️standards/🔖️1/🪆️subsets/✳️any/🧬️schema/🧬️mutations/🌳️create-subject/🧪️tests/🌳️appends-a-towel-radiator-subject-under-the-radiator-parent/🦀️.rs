//! 🧪️ `create-subject` fixture — `🌳️appends-a-towel-radiator-subject-under-the-radiator-parent`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `create-subject` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
//! the nested states `None` and `Some(None)` are NOT distinguishable in this file's committed diff,
//! and nothing here asserts that they are.

use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Mutation, Iso16757Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Iso16757Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> Iso16757Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> Iso16757Mutation {
    serde_json::from_str(MUTATION).expect("the committed `create-subject` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ With `index: None` the oracle takes its `_ => push` arm, so the new subject is appended after
/// `subject.radiator` and the `clamped` flag stays false. The subject declares `parent_id:
/// Some("subject.radiator")`, which the oracle stores verbatim without validating the parent.
#[semio_framework_async_macros::async_test]
fn appends_a_towel_radiator_subject_under_the_radiator_parent() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("create-subject applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.dictionary.subjects.len(), 2, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the subject list must grow by exactly one");
    assert_eq!(applied.dictionary.subjects[1].id, "subject.towel-radiator", "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: a null index appends, so the new subject must be last");
    assert_eq!(applied.dictionary.subjects[1].parent_id.as_deref(), Some("subject.radiator"), "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the declared parent must be stored verbatim");
}

/// ↩️ `create-subject`'s inverse is a `DeleteSubject` on the created id — but only when that id is ABSENT from
/// BASE; the fixture's id is fresh, so exactly one delete step is produced.
#[semio_framework_async_macros::async_test]
fn deleting_the_towel_radiator_subject_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward create-subject applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: creating a fresh subject inverts to exactly one DeleteSubject");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the create-subject inverse step applies");
    }
    assert_eq!(snapshot, base, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: deleting the created subject did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `create-subject` payload are already canonical: decode → encode
/// is a fixed point. The committed payload is spelled `{"CreateSubject": {"subject": {…}, "index": null}}` —
/// `index` is an `Option<usize>` with no `skip_serializing_if`, so it is present as an explicit `null`, and
/// `SubjectKind::ProductSpecialization` is spelled with its bare Rust variant name.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the create-subject payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the create-subject payload reparses");
    assert_eq!(reencoded, original, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the committed create-subject JSON is not canonical");
}

/// 🎯️ `subject.towel-radiator` is not among the committed subjects, so the fatal `mutation.duplicate-id` branch
/// is not taken; and `index` is `None`, so the `mutation.clamped` warning is not raised either.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the id is fresh (no `mutation.duplicate-id`) and the index is null rather than out of range (no `mutation.clamped`)");
    assert!(produced.messages().is_empty(), "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: an accepted create-subject emits no diagnostics at all");
}

/// 🔺️ The sparse delta `create-subject` produces is exactly the committed diff — the load-bearing assertion of
/// this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `dictionary` is rewritten
/// and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced create-subject diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// dictionary and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed create-subject diff decodes");
    let dictionary = decoded.dictionary.as_ref().expect("the committed create-subject diff carries the dictionary");
    assert_eq!(dictionary.subjects.len(), 2, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the diff carries both subjects, because the dictionary delta is whole-container");
    assert_eq!(dictionary.subjects[1].kind, crate::artifacts::iso16757::part_4::SubjectKind::ProductSpecialization, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the declared subject kind must survive the diff");
    assert!(decoded.catalogue.is_none(), "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: create-subject writes `dictionary` and must leave `catalogue` untouched");
    assert!(decoded.selection.is_none(), "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: create-subject writes `dictionary` and must leave `selection` untouched");
    assert!(decoded.part_number_inputs.is_none(), "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: create-subject writes `dictionary` and must leave `part_number_inputs` untouched");
    assert!(decoded.exchange_process.is_none(), "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: create-subject writes `dictionary` and must leave `exchange_process` untouched");
    assert!(decoded.artifact.is_none(), "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the subject creation, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed create-subject diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the committed diff did not carry before to after");
}
