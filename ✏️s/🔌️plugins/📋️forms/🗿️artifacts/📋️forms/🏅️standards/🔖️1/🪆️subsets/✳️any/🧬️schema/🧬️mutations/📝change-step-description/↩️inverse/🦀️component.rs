//! ↩️ `rename-step` / `change-step-description` — undo reconstructed from the BASE-state step;
//! missing id ⇒ `Vec::new()`.

use super::mutation::{ChangeStepDescription, ChangeStepDescription};
use crate::artifacts::forms::{FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeStepDescription, base: &FormsSnapshot) -> Vec<FormMutation> {
    match base.steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![FormMutation::ChangeStepDescription(ChangeStepDescription { id: payload.id.clone(), new_description: step.description.clone() })],
        None => Vec::new(),
    }
}
