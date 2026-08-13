//! 🔺️ Sparse diff builder for `DeleteStep` — a real cascade-aware removal (step + any edge that
//! touches it), never a whole-snapshot capture.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteStep, base: &SequenceSnapshot) -> SequenceDiff {
    let scene = sequence_working_scene(base);
    let steps = scene.steps.into_iter().filter(|step| step.id != payload.id).collect();
    let edges = scene.edges.into_iter().filter(|edge| edge.from != payload.id && edge.to != payload.id).collect();
    diff_replace_content(steps, edges)
}
//#endregion 🔖️Diff
