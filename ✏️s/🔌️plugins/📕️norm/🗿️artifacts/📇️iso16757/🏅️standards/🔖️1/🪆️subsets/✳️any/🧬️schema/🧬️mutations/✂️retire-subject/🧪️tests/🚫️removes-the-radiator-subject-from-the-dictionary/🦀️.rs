//! 🧪️ `retire-subject` fixture — `🚫️removes-the-radiator-subject-from-the-dictionary`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `retire-subject` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
//! the nested states `None` and `Some(None)` are NOT distinguishable in this file's committed diff,
//! and nothing here asserts that they are.

use crate::{Iso16757Diff, Iso16757Mutation, Iso16757Snapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️retire-subject/🚫️removes-the-radiator-subject-from-the-dictionary/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️retire-subject/🚫️removes-the-radiator-subject-from-the-dictionary/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️retire-subject/🚫️removes-the-radiator-subject-from-the-dictionary/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️retire-subject/🚫️removes-the-radiator-subject-from-the-dictionary/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️retire-subject/🚫️removes-the-radiator-subject-from-the-dictionary/🎯️outcome/🔣️.json");

fn before() -> Iso16757Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> Iso16757Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> Iso16757Mutation {
    serde_json::from_str(MUTATION).expect("the committed `retire-subject` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `retire-subject` retains everything whose id differs, emptying the list here. The catalogue's
/// `product_groups[0].dictionary_subject_id` still points at the deleted subject — the oracle performs NO
/// cascade, and this case is the pin on that: a dangling dictionary reference is a validation concern, not a
/// mutation one.
#[semio_framework_async_macros::async_test]
async fn removes_the_radiator_subject_from_the_dictionary() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("retire-subject applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "retire-subject/removes-the-radiator-subject-from-the-dictionary: the applied state differs from the committed after-snapshot");
    assert!(applied.dictionary.subjects.is_empty(), "retire-subject/removes-the-radiator-subject-from-the-dictionary: the addressed subject must be gone");
    assert_eq!(
        applied.catalogue.product_groups[0].dictionary_subject_id.as_deref(),
        Some("subject.radiator"),
        "retire-subject/removes-the-radiator-subject-from-the-dictionary: the oracle severs no catalogue reference, so the now-dangling pointer must survive verbatim"
    );
    assert_eq!(applied.dictionary.reference, before().dictionary.reference, "retire-subject/removes-the-radiator-subject-from-the-dictionary: the dictionary identity is not part of the deletion");
}

/// ↩️ `retire-subject`'s inverse is a `IntroduceSubject` carrying the removed subject AND its recorded position
/// (`index: Some(0)`), which is what makes the round trip order-exact rather than merely set-exact.
#[semio_framework_async_macros::async_test]
async fn recreating_the_radiator_subject_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward retire-subject applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "retire-subject/removes-the-radiator-subject-from-the-dictionary: deleting an existing subject inverts to exactly one positioned IntroduceSubject");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the retire-subject inverse step applies");
    }
    assert_eq!(snapshot, base, "retire-subject/removes-the-radiator-subject-from-the-dictionary: recreating the subject at its recorded index did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `retire-subject` payload are already canonical: decode → encode
/// is a fixed point. The committed payload is spelled `{"RetireSubject": {"id": "subject.radiator"}}` —
/// externally tagged, snake_case payload key.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "retire-subject/removes-the-radiator-subject-from-the-dictionary: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the retire-subject payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the retire-subject payload reparses");
    assert_eq!(reencoded, original, "retire-subject/removes-the-radiator-subject-from-the-dictionary: the committed retire-subject JSON is not canonical");
}

/// 🎯️ `subject.radiator` IS among the committed subjects, so the `mutation.target-missing` Error branch is not
/// taken.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "retire-subject/removes-the-radiator-subject-from-the-dictionary: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "retire-subject/removes-the-radiator-subject-from-the-dictionary: the addressed subject exists in the committed dictionary, so `retire-subject`'s `mutation.target-missing` error cannot fire");
    assert!(produced.messages().is_empty(), "retire-subject/removes-the-radiator-subject-from-the-dictionary: an accepted retire-subject emits no diagnostics at all");
}

/// 🔺️ The sparse delta `retire-subject` produces is exactly the committed diff — the load-bearing assertion of
/// this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `dictionary` is rewritten
/// and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced retire-subject diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "retire-subject/removes-the-radiator-subject-from-the-dictionary: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole emptied
/// dictionary and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed retire-subject diff decodes");
    let dictionary = decoded.dictionary.as_ref().expect("the committed retire-subject diff carries the dictionary");
    assert!(dictionary.subjects.is_empty(), "retire-subject/removes-the-radiator-subject-from-the-dictionary: the deletion is expressed as the shorter whole subject list");
    assert_eq!(dictionary.reference.id, "bsdd.fixture", "retire-subject/removes-the-radiator-subject-from-the-dictionary: the dictionary reference rides through the whole-container delta");
    assert!(decoded.catalogue.is_none(), "retire-subject/removes-the-radiator-subject-from-the-dictionary: retire-subject writes `dictionary` and must leave `catalogue` untouched");
    assert!(decoded.selection.is_none(), "retire-subject/removes-the-radiator-subject-from-the-dictionary: retire-subject writes `dictionary` and must leave `selection` untouched");
    assert!(decoded.part_number_rule.is_none(), "retire-subject/removes-the-radiator-subject-from-the-dictionary: retire-subject writes `dictionary` and must leave `part_number_rule` untouched");
    assert!(decoded.script_limits.is_none(), "retire-subject/removes-the-radiator-subject-from-the-dictionary: retire-subject writes `dictionary` and must leave `script_limits` untouched");
    assert!(decoded.artifact.is_none(), "retire-subject/removes-the-radiator-subject-from-the-dictionary: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "retire-subject/removes-the-radiator-subject-from-the-dictionary: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the subject deletion, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed retire-subject diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "retire-subject/removes-the-radiator-subject-from-the-dictionary: the committed diff did not carry before to after");
}
