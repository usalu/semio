//! 🔺️ Sparse diff builder for `DisconnectSteps`.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectSteps, base: &SequenceSnapshot) -> SequenceDiff {
    let scene = sequence_working_scene(base);
    let edges = scene.edges.into_iter().filter(|edge| edge.id != payload.id).collect();
    diff_replace_content(scene.steps, edges)
}
//#endregion 🔖️Diff
