//! 🔺️ Sparse diff builder for `MoveStep`.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::MoveStep, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
    let scene = sequence_working_scene(base);
    let Some(existing) = scene.steps.iter().find(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.x.is_finite() || !payload.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Step \"{}\" position must be finite, got ({}, {}).", payload.id, payload.x, payload.y), [payload.id.clone()]);
    }
    if existing.x == payload.x && existing.y == payload.y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" is already at ({}, {}).", payload.id, payload.x, payload.y));
    }
    let mut steps = scene.steps;
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.id) {
        step.x = payload.x;
        step.y = payload.y;
    }
    protocol::MutationOutcome::new(diff_replace_content(steps, scene.edges))
}
//#endregion 🔖️Diff
