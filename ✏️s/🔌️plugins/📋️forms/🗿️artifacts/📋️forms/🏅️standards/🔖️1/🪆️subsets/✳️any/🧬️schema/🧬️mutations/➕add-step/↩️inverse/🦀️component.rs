//! ↩️ `create-step` — undo is `delete-step`, unless `base` already had this id (then `create` was a
//! no-op and there's nothing to undo).

use super::mutation::CreateStep;
use crate::artifacts::forms::mutations::remove_step;
use crate::artifacts::forms::{FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_create_step(payload: &CreateStep, base: &FormsSnapshot) -> Vec<FormMutation> {
    if base.steps.iter().any(|step| step.id == payload.step.id) {
        return Vec::new();
    }
    vec![FormMutation::DeleteStep(remove_step::mutation::DeleteStep { id: payload.step.id.clone() })]
}
//#endregion 🔖️Inverse
