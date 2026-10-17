//! ↩️ `reorder-step` — undo reorders back to the BASE-state index; missing id ⇒ `Vec::new()`.

use crate::standards::v1::subsets::any::schema::mutations::reorder_step::mutation::ReorderStep;
use crate::{forms_steps, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_reorder_step<Op>(payload: &ReorderStep, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::reorder_step::mutation::ReorderStep>,
{
    Ok((|| {
    match forms_steps(base).iter().position(|step| step.id == payload.id) {
        Some(index) => vec![Op::from(ReorderStep { id: payload.id.clone(), to_index: index })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
