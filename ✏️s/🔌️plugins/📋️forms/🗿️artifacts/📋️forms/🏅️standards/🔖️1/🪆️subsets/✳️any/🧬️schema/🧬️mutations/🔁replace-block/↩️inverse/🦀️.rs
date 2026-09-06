//! ↩️ `replace-block` — undo restores the BASE-state block wholesale; missing step or block ⇒
//! `Vec::new()`.

use super::mutation::ReplaceBlock;
use crate::artifacts::forms::{forms_steps, FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_replace_block(payload: &ReplaceBlock, base: &FormsSnapshot) -> Vec<FormMutation> {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    match step.blocks.iter().find(|block| block.id == payload.block.id) {
        Some(original) => vec![FormMutation::ReplaceBlock(ReplaceBlock { step_id: payload.step_id.clone(), block: original.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
