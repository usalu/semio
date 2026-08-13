//! ↩️ `delete-step` — undo re-creates the step (with its captured blocks) at its BASE-state index;
//! missing id ⇒ `Vec::new()`.

use super::mutation::DeleteStep;
use crate::artifacts::forms::mutations::create_step;
use crate::artifacts::forms::{forms_steps, FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_delete_step(payload: &DeleteStep, base: &FormsSnapshot) -> Vec<FormMutation> {
    let steps = forms_steps(base);
    match steps.iter().position(|step| step.id == payload.id) {
        Some(index) => vec![FormMutation::CreateStep(create_step::mutation::CreateStep { step: steps[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
