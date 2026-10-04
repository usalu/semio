//! 🧪️ `set-point-positions` fixture — `🧪️restores-two-points-to-their-base-positions`.
//!
//! Source of truth is the committed JSON beside this file. Model (a) (design §20.15): the snapshots carry the graph and the
//! point cloud inline, and the `notation`/`results`/`computed` handles are the content addresses of their derivation
//! (`crate::equation_children`), kept exact by the fixture writer law. The row is the exact undo of the `move-points`
//! `🧪️translates` drag: its `before` is that drag's `after` and its `after` that drag's `before`.

use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📌️set-point-positions/🧪️restores/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📌️set-point-positions/🧪️restores/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📌️set-point-positions/🧪️restores/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📌️set-point-positions/🧪️restores/🔺️diff/🔣️.json");
const DRAG_BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/📸️snapshot/⬅️before/🔣️.json");
const DRAG_AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/📸️snapshot/➡️after/🔣️.json");

fn snapshot(text: &str) -> EquationSnapshot {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &snapshot(BEFORE))
}

/// ▶️ The row carries `before` to exactly the committed `after` through exactly the committed diff, with a clean `applied`.
#[semio_framework_async_macros::async_test]
async fn applies_the_committed_diff_to_the_committed_after() {
    let outcome = produced();
    assert!(outcome.messages().is_empty(), "an in-range placement raises no diagnostic, got {:?}", outcome.messages());
    let committed: EquationDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(outcome.diff(), &committed, "set-point-positions/restores: produced diff differs from the committed 🔺️diff");
    let applied = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(outcome.diff(), &snapshot(BEFORE)).expect("the placement applies");
    assert_eq!(applied, snapshot(AFTER), "set-point-positions/restores: applied state differs from the committed after-snapshot");
}

/// 🔁️ The vector is the exact mirror of the drag it undoes, and its own inverse is one row that re-applies the drag.
#[semio_framework_async_macros::async_test]
async fn it_mirrors_the_drag_and_is_point_invertible() {
    assert_eq!((snapshot(BEFORE), snapshot(AFTER)), (snapshot(DRAG_AFTER), snapshot(DRAG_BEFORE)), "set-point-positions/restores undoes exactly move-points/translates");
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &snapshot(BEFORE)).expect("valid retained mutation inverse fixture");
    assert!(matches!(inverse.as_slice(), [EquationMutation::SetPointPositions(_)]), "the inverse is ONE absolute row, got {inverse:?}");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&snapshot(BEFORE), &mutation()).await;
}

/// 🔣️ Every committed document is canonical, and its handles are the content addresses of its own state.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical_and_content_addressed() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded = snapshot(text);
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&semio_framework_pack_json::from_dsl_value(&decoded.to_value()), &original), "set-point-positions/restores: committed {label} JSON is not canonical");
        assert_eq!((decoded.notation.clone(), decoded.results.clone(), decoded.computed.clone()), crate::equation_children(&decoded.graph, &decoded.geometry), "set-point-positions/restores: committed {label} handles are not the content addresses of its state");
    }
    let original = semio_framework_pack_json::parse(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&semio_framework_pack_json::from_dsl_value(&mutation().to_value()), &original), "set-point-positions/restores: committed mutation JSON is not canonical");
}
