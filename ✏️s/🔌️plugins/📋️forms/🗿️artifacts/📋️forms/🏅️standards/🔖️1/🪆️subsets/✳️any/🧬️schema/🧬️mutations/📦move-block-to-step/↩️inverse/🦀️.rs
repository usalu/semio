//! ↩️ `move-block-to-step` — undo moves the block back from its new (destination) step to its
//! BASE-state step, at its BASE-state index; missing source step or block ⇒ `Vec::new()`.

use crate::standards::v1::subsets::any::schema::mutations::move_block_to_step::mutation::MoveBlockToStep;
use crate::{forms_steps, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_move_block_to_step<Op>(payload: &MoveBlockToStep, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::move_block_to_step::mutation::MoveBlockToStep>,
{
    Ok((|| {
    let steps = forms_steps(base);
    let Some(source_step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    let Some(original_index) = source_step.blocks.iter().position(|block| block.id == payload.block_id) else {
        return Vec::new();
    };
    vec![Op::from(MoveBlockToStep { step_id: payload.to_step_id.clone(), block_id: payload.block_id.clone(), to_step_id: payload.step_id.clone(), index: original_index })]

    })())
}
//#endregion 🔖️Inverse
