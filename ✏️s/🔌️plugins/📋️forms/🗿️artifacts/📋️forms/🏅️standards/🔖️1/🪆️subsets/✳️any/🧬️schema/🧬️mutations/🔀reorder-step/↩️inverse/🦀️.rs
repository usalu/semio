//! ↩️ `reorder-step` — undo reorders back to the BASE-state index; missing id ⇒ `Vec::new()`.

use super::mutation::ReorderStep;
use crate::{forms_steps, FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_reorder_step(payload: &ReorderStep, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match forms_steps(base).iter().position(|step| step.id == payload.id) {
        Some(index) => vec![FormMutation::ReorderStep(ReorderStep { id: payload.id.clone(), to_index: index })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
