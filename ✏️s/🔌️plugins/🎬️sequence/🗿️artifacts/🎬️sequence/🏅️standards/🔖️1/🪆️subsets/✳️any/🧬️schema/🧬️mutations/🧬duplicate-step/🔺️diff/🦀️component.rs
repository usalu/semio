//! 🔺️ Sparse diff builder for `DuplicateStep` — a real copy-from-BASE insert (never a
//! whole-snapshot capture). Missing source ⇒ empty diff.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceSnapshot, SequenceStep};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DuplicateStep, base: &SequenceSnapshot) -> SequenceDiff {
    let scene = sequence_working_scene(base);
    let Some(source) = scene.steps.iter().find(|step| step.id == payload.source_id) else {
        return SequenceDiff::default();
    };
    let copy = SequenceStep { id: payload.new_id.clone(), kind: source.kind.clone(), params: source.params.clone(), x: payload.x, y: payload.y, slot: None, collapsed: source.collapsed };
    let mut steps = scene.steps;
    steps.push(copy);
    diff_replace_content(steps, scene.edges)
}
//#endregion 🔖️Diff
