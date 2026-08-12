//! ↩️ `reorder-step` — undo reorders back to the BASE-state index; missing id ⇒ `Vec::new()`.

use super::mutation::ReorderStep;
use crate::artifacts::forms::{FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_reorder_step(payload: &ReorderStep, base: &FormsSnapshot) -> Vec<FormMutation> {
    match base.steps.iter().position(|step| step.id == payload.id) {
        Some(index) => vec![FormMutation::ReorderStep(ReorderStep { id: payload.id.clone(), to_index: index })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
