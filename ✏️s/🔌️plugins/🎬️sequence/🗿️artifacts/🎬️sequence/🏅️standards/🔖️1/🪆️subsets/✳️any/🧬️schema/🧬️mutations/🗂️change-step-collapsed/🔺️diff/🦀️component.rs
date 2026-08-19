//! 🔺️ Sparse diff builder for `ChangeStepCollapsed`.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ChangeStepCollapsed, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
    let scene = sequence_working_scene(base);
    let Some(existing) = scene.steps.iter().find(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.collapsed == payload.collapsed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" collapsed is already {}.", payload.id, payload.collapsed));
    }
    let mut steps = scene.steps;
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.id) {
        step.collapsed = payload.collapsed;
    }
    protocol::MutationOutcome::new(diff_replace_content(steps, scene.edges))
}
//#endregion 🔖️Diff
