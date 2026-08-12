//! ↩️ `rename-step` / `change-step-description` — undo reconstructed from the BASE-state step;
//! missing id ⇒ `Vec::new()`.

use super::mutation::{ChangeStepDescription, RenameStep};
use crate::artifacts::forms::{FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_rename_step(payload: &RenameStep, base: &FormsSnapshot) -> Vec<FormMutation> {
    match base.steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![FormMutation::RenameStep(RenameStep { id: payload.id.clone(), new_title: step.title.clone() })],
        None => Vec::new(),
    }
}

pub fn inverse_change_step_description(payload: &ChangeStepDescription, base: &FormsSnapshot) -> Vec<FormMutation> {
    match base.steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![FormMutation::ChangeStepDescription(ChangeStepDescription { id: payload.id.clone(), new_description: step.description.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
