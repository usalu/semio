//! ↩️ Inverse for `EditStepParams` — the OLD params body looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::EditStepParams, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    match base.steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![super::mutation::edit_step_params(payload.id.clone(), step.params.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
