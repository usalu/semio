//! ⏪️ `move-nodes` history law (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING audit G1): a released equation-graph node drag
//! is ONE relative `move-nodes` leaf; edited in history — re-offset or re-targeted — its Report replay re-applies the downstream
//! drag onto the edited one exactly like a fresh fold of the edited log, through the cross-plugin
//! `semio_framework_plugin::app::node_drag_history` law.

use crate::standards::v1::subsets::graph::schema::mutations::{create_node::CreateNode, move_nodes::MoveNodes};
use crate::{EquationMutation, EquationSnapshot, MATH_DOCUMENT_SCHEMA};

fn drag(ids: &[&str], dx: f64, dy: f64) -> EquationMutation {
    EquationMutation::MoveNodes(MoveNodes { ids: ids.iter().map(|id| id.to_string()).collect(), dx, dy })
}

fn fold(state: &mut EquationSnapshot, leaf: &EquationMutation) {
    *state = store::apply_mutation(state, leaf).expect("the logged leaf applies").0;
}

/// ⚖️ LAW: a drag re-offset in history and a drag re-targeted in history each replay the downstream drag onto the edited one
/// exactly like a fresh fold of the edited log, and overwrite to that head.
#[semio_framework_async_macros::async_test]
async fn an_edited_drag_replays_its_downstream_like_a_fresh_fold() {
    let mut base = EquationSnapshot::default();
    for (id, x, y) in [("alpha", 40.0, 60.0), ("beta", 200.0, 60.0)] {
        fold(&mut base, &EquationMutation::CreateNode(CreateNode { id: id.into(), label: id.into(), x, y, index: None }));
    }
    let log = [drag(&["alpha"], 20.0, -10.0), drag(&["alpha", "beta"], 5.0, 5.0)];
    let edits = [(0, drag(&["alpha"], -30.0, 12.5)), (0, drag(&["alpha", "beta"], 20.0, -10.0))];
    semio_framework_plugin::app::node_drag_history::assert_node_drag_edits_replay_like_a_fresh_fold("mathematical", MATH_DOCUMENT_SCHEMA, &base, &log, &edits, fold).await;
}
