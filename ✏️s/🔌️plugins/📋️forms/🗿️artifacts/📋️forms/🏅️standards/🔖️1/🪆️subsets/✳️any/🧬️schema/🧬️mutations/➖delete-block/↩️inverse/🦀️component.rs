//! ↩️ `delete-block` — undo re-creates the block at its BASE-state index within its step; missing
//! step or block ⇒ `Vec::new()`.

use super::mutation::DeleteBlock;
use crate::artifacts::forms::mutations::add_block;
use crate::artifacts::forms::{FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_delete_block(payload: &DeleteBlock, base: &FormsSnapshot) -> Vec<FormMutation> {
    let Some(step) = base.steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    match step.blocks.iter().position(|block| block.id == payload.id) {
        Some(index) => vec![FormMutation::CreateBlock(add_block::mutation::CreateBlock {
            step_id: payload.step_id.clone(),
            block: step.blocks[index].clone(),
            index: Some(index),
        })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
