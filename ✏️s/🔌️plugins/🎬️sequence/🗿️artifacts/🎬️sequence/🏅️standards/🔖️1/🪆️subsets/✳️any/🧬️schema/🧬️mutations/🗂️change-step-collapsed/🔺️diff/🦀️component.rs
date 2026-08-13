//! 🔺️ Sparse diff builder for `ChangeStepCollapsed`.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeStepCollapsed, base: &SequenceSnapshot) -> SequenceDiff {
    let scene = sequence_working_scene(base);
    let mut steps = scene.steps;
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.id) {
        step.collapsed = payload.collapsed;
    }
    diff_replace_content(steps, scene.edges)
}
//#endregion 🔖️Diff
