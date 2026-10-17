//! ↩️ `replace-block` — undo restores the BASE-state block wholesale; missing step or block ⇒
//! `Vec::new()`.

use crate::standards::v1::subsets::any::schema::mutations::replace_block::mutation::ReplaceBlock;
use crate::{forms_steps, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_replace_block<Op>(payload: &ReplaceBlock, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_block::mutation::ReplaceBlock>,
{
    Ok((|| {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    match step.blocks.iter().find(|block| block.id == payload.block.id) {
        Some(original) => vec![Op::from(ReplaceBlock { step_id: payload.step_id.clone(), block: original.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
