//! ↩️ `reorder-step` — undo reorders back to the BASE-state index; missing id ⇒ `Vec::new()`.

use super::mutation::ReorderStep;
use crate::artifacts::forms::{forms_steps, FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub async fn inverse_reorder_step(payload: &ReorderStep, base: &FormsSnapshot) -> Vec<FormMutation> {
    match forms_steps(base).iter().position(|step| step.id == payload.id) {
        Some(index) => vec![FormMutation::ReorderStep(ReorderStep { id: payload.id.clone(), to_index: index })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
