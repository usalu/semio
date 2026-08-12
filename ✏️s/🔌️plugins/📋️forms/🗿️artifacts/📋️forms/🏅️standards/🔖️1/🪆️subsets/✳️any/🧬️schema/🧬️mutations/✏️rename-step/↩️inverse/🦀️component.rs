//! ↩️ `rename-step` / `change-step-description` — undo reconstructed from the BASE-state step;
//! missing id ⇒ `Vec::new()`.

use super::mutation::{ChangeStepDescription, RenameStep};
use crate::artifacts::forms::{FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RenameStep, base: &FormsSnapshot) -> Vec<FormMutation> {
    match base.steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![FormMutation::RenameStep(RenameStep { id: payload.id.clone(), new_title: step.title.clone() })],
        None => Vec::new(),
    }
}
