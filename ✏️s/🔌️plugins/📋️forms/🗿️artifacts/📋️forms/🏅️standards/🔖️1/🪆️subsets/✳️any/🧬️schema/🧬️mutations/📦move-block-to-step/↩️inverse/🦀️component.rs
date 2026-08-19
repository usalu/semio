//! ↩️ `move-block-to-step` — undo moves the block back from its new (destination) step to its
//! BASE-state step, at its BASE-state index; missing source step or block ⇒ `Vec::new()`.

use super::mutation::MoveBlockToStep;
use crate::artifacts::forms::{forms_steps, FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub async fn inverse_move_block_to_step(payload: &MoveBlockToStep, base: &FormsSnapshot) -> Vec<FormMutation> {
    let steps = forms_steps(base);
    let Some(source_step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    let Some(original_index) = source_step.blocks.iter().position(|block| block.id == payload.block_id) else {
        return Vec::new();
    };
    vec![FormMutation::MoveBlockToStep(MoveBlockToStep {
        step_id: payload.to_step_id.clone(),
        block_id: payload.block_id.clone(),
        to_step_id: payload.step_id.clone(),
        index: original_index,
    })]
}
//#endregion 🔖️Inverse
