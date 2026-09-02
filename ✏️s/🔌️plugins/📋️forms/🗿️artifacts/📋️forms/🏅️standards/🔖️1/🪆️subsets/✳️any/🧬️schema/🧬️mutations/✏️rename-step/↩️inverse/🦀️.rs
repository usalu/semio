//! ↩️ `rename-step` / `change-step-description` — undo reconstructed from the BASE-state step;
//! missing id ⇒ `Vec::new()`.

use super::mutation::RenameStep;
use crate::artifacts::forms::{forms_steps, FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &RenameStep, base: &FormsSnapshot) -> Vec<FormMutation> {
    match forms_steps(base).iter().find(|step| step.id == payload.id) {
        Some(step) => vec![FormMutation::RenameStep(RenameStep { id: payload.id.clone(), new_title: step.title.clone() })],
        None => Vec::new(),
    }
}
