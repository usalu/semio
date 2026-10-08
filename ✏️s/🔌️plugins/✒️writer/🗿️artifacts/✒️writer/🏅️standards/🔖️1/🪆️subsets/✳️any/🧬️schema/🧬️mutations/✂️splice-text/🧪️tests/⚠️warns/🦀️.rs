//! 🧪️ `splice-text` fixture — `⚠️warns`.
//!
//! The author saw `Draft: ` between the heading and the first sentence and deleted it; a co-author had
//! already removed it, so the run is gone from the body this splice reaches. `SpliceText` never deletes
//! text its author did not see: the located run is empty (`clamped`), the insert is empty too, the body
//! is unchanged, and the diff oracle answers the same Warning `mutation.no-op` as an unchanged
//! `edit-text` — above all without re-minting the content-addressed `document` handle.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the relocation rule is `semio.ui.scene.text-splice.v1`
//! (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧬️schema/✂️text-splice/🔣️.json`).

use crate::schema::mutations::{apply_writer_mutation, inverse_writer_mutation, WriterMutation};
use crate::WriterDiff;
use crate::WriterSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️splice-text/⚠️warns/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️splice-text/⚠️warns/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️splice-text/⚠️warns/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️splice-text/⚠️warns/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️splice-text/⚠️warns/🎯️outcome/🔣️.json");

fn before() -> WriterSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before writer document decodes")
}
fn expected_after() -> WriterSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after writer document decodes")
}
fn mutation() -> WriterMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("splice-text mutation decodes")
}

/// ▶️ Deleting a run that is already gone deletes nothing: the committed after-snapshot is the
/// before-snapshot, and the content-addressed `document` handle is the very same handle.
#[semio_framework_async_macros::async_test]
async fn a_vanished_run_deletes_nothing_and_keeps_the_handle() {
    let base = before();
    let WriterMutation::SpliceText(splice) = mutation() else {
        panic!("splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: the committed payload must be a splice-text");
    };
    assert!(!base.text.contains(&splice.deleted), "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: the committed body must no longer hold the deleted run");
    let mut snapshot = base.clone();
    apply_writer_mutation(&mut snapshot, &mutation()).expect("splice-text applies to its committed before-document");
    assert_eq!(snapshot, expected_after(), "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: the clamped splice must reproduce the committed after-snapshot");
    assert_eq!(snapshot.document.child_id, base.document.child_id, "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: an unchanged body must not mint a new content address");
}

/// ↩️ The inverse is the located inverse splice: nothing was removed and nothing inserted, so it is an
/// empty splice at the same place, and applying it lands back on the before-document.
#[semio_framework_async_macros::async_test]
async fn the_inverse_is_the_empty_located_splice() {
    let base = before();
    let inverse = inverse_writer_mutation(&base, &mutation()).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: undoing a splice is exactly one splice back");
    let WriterMutation::SpliceText(undo) = &inverse[0] else {
        panic!("splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: splice-text's inverse must be a splice-text");
    };
    assert_eq!((undo.start, undo.deleted.as_str(), undo.insert.as_str()), (17, "", ""), "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: the inverse of a clamped empty splice removes and restores nothing");
    let mut snapshot = base.clone();
    apply_writer_mutation(&mut snapshot, &mutation()).expect("forward splice-text applies");
    for step in &inverse {
        apply_writer_mutation(&mut snapshot, step).expect("the splice-text inverse step applies");
    }
    assert_eq!(snapshot, base, "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: undoing a no-op must still land back on the before-document");
}

/// 🔣️ Both committed documents and the `spliceText` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WriterSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("writer document decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("writer document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("writer document reparses");
        assert_eq!(reencoded, original, "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(mutation()))).expect("spliceText payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("spliceText payload reparses");
    assert_eq!(reencoded, original, "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: committed spliceText JSON is not canonical");
}

/// 🎯️ An unchanged body is `no-op` with a single Warning: the no-op check precedes the clamp report, so
/// `mutation.no-op` is the one diagnostic.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("no-op"), "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: a no-op is its own outcome class, not a rejection");
    let produced = <WriterMutation as protocol::Mutation<WriterSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), Some(semio_framework_diagnostic::Severity::Warning), "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: an unchanged body is a Warning, never an Error");
    assert_eq!(produced.messages().len(), 1, "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: exactly one diagnostic is raised");
    assert_eq!(produced.messages()[0].code.0.as_str(), declared["messages"][0]["code"].as_str().expect("declared message code is a string"), "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: raised diagnostic code differs from the declared one");
}

/// 🔺️ The committed diff is `WriterDiff`'s all-null default, and it is what the oracle produces.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <WriterMutation as protocol::Mutation<WriterSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced splice-text diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: produced diff differs from the committed 🔺️diff/🔣️.json");
    let decoded: WriterDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed splice-text diff decodes");
    assert_eq!(decoded, WriterDiff::default(), "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: a no-op's committed diff must be the type's own default");
    let applied = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(applied, expected_after(), "splice-text/warns-that-an-already-removed-run-leaves-the-brief-unchanged: committed diff did not carry before to after");
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
