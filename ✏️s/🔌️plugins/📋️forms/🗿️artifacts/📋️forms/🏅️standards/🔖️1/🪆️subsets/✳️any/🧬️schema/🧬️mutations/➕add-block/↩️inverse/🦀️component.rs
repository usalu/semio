//! ↩️ `create-block` — undo is `delete-block`, unless `base`'s step already had this block id (then
//! `create` was a no-op) or the step itself no longer exists.

use super::mutation::CreateBlock;
use crate::artifacts::forms::mutations::remove_block;
use crate::artifacts::forms::{FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_create_block(payload: &CreateBlock, base: &FormsSnapshot) -> Vec<FormMutation> {
    let Some(step) = base.steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    if step.blocks.iter().any(|block| block.id == payload.block.id) {
        return Vec::new();
    }
    vec![FormMutation::DeleteBlock(remove_block::mutation::DeleteBlock { step_id: payload.step_id.clone(), id: payload.block.id.clone() })]
}
//#endregion 🔖️Inverse
