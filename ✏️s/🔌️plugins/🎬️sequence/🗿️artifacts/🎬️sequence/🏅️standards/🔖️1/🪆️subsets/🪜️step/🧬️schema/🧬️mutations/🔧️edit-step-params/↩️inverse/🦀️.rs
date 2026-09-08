//! ↩️ Inverse for `EditStepParams` — the OLD params body looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::mutations::SequenceMutation;
use crate::{sequence_working_scene, SequenceSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::EditStepParams, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    let scene = sequence_working_scene(base);
    match scene.steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![super::edit_step_params(payload.id.clone(), step.params.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
