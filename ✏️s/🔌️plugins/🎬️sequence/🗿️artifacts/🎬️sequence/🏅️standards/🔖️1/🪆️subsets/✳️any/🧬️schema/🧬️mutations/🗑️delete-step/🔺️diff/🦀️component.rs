//! 🔺️ Sparse diff builder for `DeleteStep` — a real cascade-aware removal (step + any edge that
//! touches it), never a whole-snapshot capture.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteStep, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
    let scene = sequence_working_scene(base);
    if !scene.steps.iter().any(|step| step.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let cascaded_edge_ids: Vec<String> = scene.edges.iter().filter(|edge| edge.from == payload.id || edge.to == payload.id).map(|edge| edge.id.clone()).collect();
    let steps = scene.steps.into_iter().filter(|step| step.id != payload.id).collect();
    let edges = scene.edges.into_iter().filter(|edge| edge.from != payload.id && edge.to != payload.id).collect();
    let outcome = protocol::MutationOutcome::new(diff_replace_content(steps, edges));
    if cascaded_edge_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting step \"{}\" also removed {} connected edge(s): {}.", payload.id, cascaded_edge_ids.len(), cascaded_edge_ids.join(", ")))
    }
}
//#endregion 🔖️Diff
