//! ↩️ `delete-block` — undo re-creates the block at its BASE-state index within its step; missing
//! step or block ⇒ `Vec::new()`.

use crate::standards::v1::subsets::any::schema::mutations::delete_block::mutation::DeleteBlock;
use crate::mutations::create_block::mutation::CreateBlock;
use crate::{forms_steps, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_delete_block<Op>(payload: &DeleteBlock, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_block::mutation::CreateBlock>,
{
    Ok((|| {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    match step.blocks.iter().position(|block| block.id == payload.id) {
        Some(index) => vec![Op::from(CreateBlock { step_id: payload.step_id.clone(), block: step.blocks[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
