//! 🔺️ Sparse diff builder for `DisconnectSteps`.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DisconnectSteps, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
    let scene = sequence_working_scene(base);
    if !scene.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let edges = scene.edges.into_iter().filter(|edge| edge.id != payload.id).collect();
    protocol::MutationOutcome::new(diff_replace_content(scene.steps, edges))
}
//#endregion 🔖️Diff
