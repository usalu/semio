//! ⏪️ Sequence node-drag history law (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING audit G1): a released step drag is ONE
//! relative `drag-nodes` leaf on the composed `content` child (design §12); edited in history — re-offset or re-targeted — the
//! child's Report replay re-applies the downstream drag onto the edited one exactly like a fresh fold of the edited log, through
//! the cross-plugin `semio_framework_plugin::app::node_drag_history` law, on the content of the default sequence.

use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{drag_nodes::DragNodes, SemioFlowMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};

fn drag(targets: &[&str], dx: f64, dy: f64) -> SemioFlowMutation {
    SemioFlowMutation::DragNodes(DragNodes { targets: targets.iter().map(|id| id.to_string()).collect(), dx, dy })
}

fn fold(state: &mut SemioFlowSnapshot, leaf: &SemioFlowMutation) {
    *state = store::apply_mutation(state, leaf).expect("the logged leaf applies").0;
}

/// ⚖️ LAW: a step drag re-offset in history and one re-targeted in history each replay the downstream drag onto the edited one
/// exactly like a fresh fold of the edited log, and overwrite to that head.
#[semio_framework_async_macros::async_test]
async fn an_edited_step_drag_replays_its_downstream_like_a_fresh_fold() {
    let content = crate::default_host_snapshot();
    let base = crate::sequence_content_snapshot_from_working(&content.steps, &content.edges);
    let log = [drag(&["step-1"], 20.0, -10.0), drag(&["step-1", "step-2"], 5.0, 5.0)];
    let edits = [(0, drag(&["step-1"], -30.0, 12.5)), (0, drag(&["step-1", "step-2"], 20.0, -10.0))];
    semio_framework_plugin::app::node_drag_history::assert_node_drag_edits_replay_like_a_fresh_fold("sequence", STDIO_SEMIOFLOW_DOCUMENT_SCHEMA, &base, &log, &edits, fold).await;
}
