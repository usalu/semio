//! 🔺️ Sparse diff builder for `ConnectSteps`.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceEdge, SequenceSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectSteps, base: &SequenceSnapshot) -> SequenceDiff {
    let scene = sequence_working_scene(base);
    let mut edges = scene.edges;
    edges.push(SequenceEdge { id: payload.id.clone(), from: payload.from.clone(), to: payload.to.clone() });
    diff_replace_content(scene.steps, edges)
}
//#endregion 🔖️Diff
