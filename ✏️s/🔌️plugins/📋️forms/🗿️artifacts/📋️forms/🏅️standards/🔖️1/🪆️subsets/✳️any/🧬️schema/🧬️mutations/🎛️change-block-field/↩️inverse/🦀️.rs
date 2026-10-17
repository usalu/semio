//! ↩️ `change-block-field` — undo sets the same field back to the BASE question's value; a missing question ⇒ `Vec::new()`.

use crate::standards::v1::subsets::any::schema::mutations::change_block_field::mutation::ChangeBlockField;
use crate::{forms_steps, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_change_block_field<Op>(payload: &ChangeBlockField, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_block_field::mutation::ChangeBlockField>,
{
    Ok((|| {
    forms_steps(base)
        .iter()
        .flat_map(|step| step.blocks.iter())
        .find(|block| block.id == payload.block_id)
        .map(|original| Op::from(ChangeBlockField { block_id: payload.block_id.clone(), change: payload.change.read(original) }))
        .into_iter()
        .collect()

    })())
}
//#endregion 🔖️Inverse
