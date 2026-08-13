//! ↩️ Inverse for `ChangeStepCollapsed` — the OLD `collapsed` looked up from BASE. Missing target
//! ⇒ `Vec::new()`.
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::{sequence_working_scene, SequenceSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeStepCollapsed, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    let scene = sequence_working_scene(base);
    match scene.steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![super::mutation::change_step_collapsed(payload.id.clone(), step.collapsed)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
