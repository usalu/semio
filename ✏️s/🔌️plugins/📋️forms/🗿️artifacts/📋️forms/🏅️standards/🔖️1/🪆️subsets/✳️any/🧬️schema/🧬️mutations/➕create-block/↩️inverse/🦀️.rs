//! ↩️ `create-block` — undo is `delete-block`, unless `base`'s step already had this block id (then
//! `create` was a no-op) or the step itself no longer exists.

use super::mutation::CreateBlock;
use crate::artifacts::forms::mutations::delete_block;
use crate::artifacts::forms::{forms_steps, FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub async fn inverse_create_block(payload: &CreateBlock, base: &FormsSnapshot) -> Vec<FormMutation> {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    if step.blocks.iter().any(|block| block.id == payload.block.id) {
        return Vec::new();
    }
    vec![FormMutation::DeleteBlock(delete_block::mutation::DeleteBlock { step_id: payload.step_id.clone(), id: payload.block.id.clone() })]
}
//#endregion 🔖️Inverse
