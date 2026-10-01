//! ↩️ `change-block-field` — undo sets the same field back to the BASE question's value; a missing question ⇒ `Vec::new()`.

use super::mutation::ChangeBlockField;
use crate::{forms_steps, FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_change_block_field(payload: &ChangeBlockField, base: &FormsSnapshot) -> Vec<FormMutation> {
    forms_steps(base)
        .iter()
        .flat_map(|step| step.blocks.iter())
        .find(|block| block.id == payload.block_id)
        .map(|original| FormMutation::ChangeBlockField(ChangeBlockField { block_id: payload.block_id.clone(), change: payload.change.read(original) }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
