//! ↩️ `rename-step` / `change-step-description` — undo reconstructed from the BASE-state step;
//! missing id ⇒ `Vec::new()`.

use super::mutation::ChangeStepDescription;
use crate::artifacts::forms::{forms_steps, FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeStepDescription, base: &FormsSnapshot) -> Vec<FormMutation> {
    match forms_steps(base).iter().find(|step| step.id == payload.id) {
        Some(step) => vec![FormMutation::ChangeStepDescription(ChangeStepDescription { id: payload.id.clone(), new_description: step.description.clone() })],
        None => Vec::new(),
    }
}
