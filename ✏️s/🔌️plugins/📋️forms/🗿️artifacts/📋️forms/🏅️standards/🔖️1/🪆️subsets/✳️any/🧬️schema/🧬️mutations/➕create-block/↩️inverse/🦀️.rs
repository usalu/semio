//! ↩️ `create-block` — undo is `delete-block`, unless `base`'s step already had this block id (then
//! `create` was a no-op) or the step itself no longer exists.

use crate::standards::v1::subsets::any::schema::mutations::create_block::mutation::CreateBlock;
use crate::mutations::delete_block::mutation::DeleteBlock;
use crate::{forms_steps, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_create_block<Op>(payload: &CreateBlock, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_block::mutation::DeleteBlock>,
{
    Ok((|| {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    if step.blocks.iter().any(|block| block.id == payload.block.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteBlock { step_id: payload.step_id.clone(), id: payload.block.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
