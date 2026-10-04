//! ↩️ `rename-step` / `change-step-description` — undo reconstructed from the BASE-state step;
//! missing id ⇒ `Vec::new()`.

use super::mutation::ChangeStepDescription;
use crate::{forms_steps, FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeStepDescription, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match forms_steps(base).iter().find(|step| step.id == payload.id) {
        Some(step) => vec![FormMutation::ChangeStepDescription(ChangeStepDescription { id: payload.id.clone(), new_description: step.description.clone() })],
        None => Vec::new(),
    }

    })())
}
