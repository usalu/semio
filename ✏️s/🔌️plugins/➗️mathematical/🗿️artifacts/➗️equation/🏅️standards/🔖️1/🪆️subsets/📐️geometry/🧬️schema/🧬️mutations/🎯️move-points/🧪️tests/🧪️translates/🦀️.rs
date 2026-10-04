//! 🧪️ `move-points` fixture — `🧪️translates-two-points-by-the-drag-offset`.
//!
//! Source of truth is the committed JSON beside this file. Model (a) (design §20.15): the snapshots carry the graph and the
//! point cloud inline, and the `notation`/`results`/`computed` handles are the content addresses of their derivation
//! (`crate::equation_children`), kept exact by the fixture writer law. The drag moves points 0 and 2 by `(5, -2.5)` from their
//! BASE positions and leaves point 1 where it is.

use crate::standards::v1::subsets::geometry::schema::mutations::set_point_positions::{EquationPointPosition, SetPointPositions};
use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/🎯️outcome/🔣️.json");

fn snapshot(text: &str) -> EquationSnapshot {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn committed_diff() -> EquationDiff {
    semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &snapshot(BEFORE))
}

/// ▶️ The drag carries `before` to exactly the committed `after` through exactly the committed diff, with a clean `applied`.
#[semio_framework_async_macros::async_test]
async fn applies_the_committed_diff_to_the_committed_after() {
    let outcome = produced();
    assert!(outcome.messages().is_empty(), "an in-range drag raises no diagnostic, got {:?}", outcome.messages());
    assert_eq!(outcome.diff(), &committed_diff(), "move-points/translates: produced diff differs from the committed 🔺️diff");
    let applied = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(outcome.diff(), &snapshot(BEFORE)).expect("the drag applies");
    assert_eq!(applied, snapshot(AFTER), "move-points/translates: applied state differs from the committed after-snapshot");
    let declared = semio_framework_pack_json::parse(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("applied"));
}

/// ↩️ The undo is ONE absolute `set-point-positions` row carrying every moved point's BASE position, and it restores `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_is_one_absolute_row_that_restores_before() {
    let base = snapshot(BEFORE);
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    let expected = SetPointPositions { positions: [0, 2].map(|index| EquationPointPosition { index, x: base.geometry.points[index].x, y: base.geometry.points[index].y }).to_vec() };
    assert_eq!(inverse, vec![EquationMutation::SetPointPositions(expected)]);
    let mut state = snapshot(AFTER);
    for step in &inverse {
        let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(step, &state);
        state = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(outcome.diff(), &state).expect("the inverse applies");
    }
    assert_eq!(state, base, "move-points/translates: the inverse did not restore the before-snapshot");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation()).await;
}

/// 🔣️ Every committed document is canonical, and its handles are the content addresses of its own state.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical_and_content_addressed() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded = snapshot(text);
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&semio_framework_pack_json::from_dsl_value(&decoded.to_value()), &original), "move-points/translates: committed {label} JSON is not canonical");
        assert_eq!((decoded.notation.clone(), decoded.results.clone(), decoded.computed.clone()), crate::equation_children(&decoded.graph, &decoded.geometry), "move-points/translates: committed {label} handles are not the content addresses of its state");
    }
    let original = semio_framework_pack_json::parse(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&semio_framework_pack_json::from_dsl_value(&mutation().to_value()), &original), "move-points/translates: committed mutation JSON is not canonical");
}
