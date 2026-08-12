//! ↩️ `delete-step` — undo re-creates the step (with its captured blocks) at its BASE-state index;
//! missing id ⇒ `Vec::new()`.

use super::mutation::DeleteStep;
use crate::artifacts::forms::mutations::add_step;
use crate::artifacts::forms::{FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_delete_step(payload: &DeleteStep, base: &FormsSnapshot) -> Vec<FormMutation> {
    match base.steps.iter().position(|step| step.id == payload.id) {
        Some(index) => vec![FormMutation::CreateStep(add_step::mutation::CreateStep { step: base.steps[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
