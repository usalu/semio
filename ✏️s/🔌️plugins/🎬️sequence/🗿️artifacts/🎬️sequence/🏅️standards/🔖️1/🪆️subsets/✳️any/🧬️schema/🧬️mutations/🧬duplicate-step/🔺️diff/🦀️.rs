//! 🔺️ Sparse diff builder for `DuplicateStep` — a real copy-from-BASE insert (never a
//! whole-snapshot capture). Missing source ⇒ empty diff.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceSnapshot, SequenceStep};

//#region 🔖️Diff
pub async fn diff(payload: &super::DuplicateStep, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
    let scene = sequence_working_scene(base);
    let Some(source) = scene.steps.iter().find(|step| step.id == payload.source_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.source_id), [payload.source_id.clone()]);
    };
    if scene.steps.iter().any(|step| step.id == payload.new_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A step with id \"{}\" already exists.", payload.new_id), [payload.new_id.clone()]);
    }
    let copy = SequenceStep { id: payload.new_id.clone(), kind: source.kind.clone(), params: source.params.clone(), x: payload.x, y: payload.y, slot: None, collapsed: source.collapsed };
    let mut steps = scene.steps;
    steps.push(copy);
    protocol::MutationOutcome::new(diff_replace_content(steps, scene.edges))
}
//#endregion 🔖️Diff
