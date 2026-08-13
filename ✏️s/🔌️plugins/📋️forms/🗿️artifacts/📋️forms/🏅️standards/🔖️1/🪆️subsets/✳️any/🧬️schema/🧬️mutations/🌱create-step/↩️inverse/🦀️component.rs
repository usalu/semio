//! ↩️ `create-step` — undo is `delete-step`, unless `base` already had this id (then `create` was a
//! no-op and there's nothing to undo).

use super::mutation::CreateStep;
use crate::artifacts::forms::mutations::delete_step;
use crate::artifacts::forms::{forms_steps, FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_create_step(payload: &CreateStep, base: &FormsSnapshot) -> Vec<FormMutation> {
    if forms_steps(base).iter().any(|step| step.id == payload.step.id) {
        return Vec::new();
    }
    vec![FormMutation::DeleteStep(delete_step::mutation::DeleteStep { id: payload.step.id.clone() })]
}
//#endregion 🔖️Inverse
