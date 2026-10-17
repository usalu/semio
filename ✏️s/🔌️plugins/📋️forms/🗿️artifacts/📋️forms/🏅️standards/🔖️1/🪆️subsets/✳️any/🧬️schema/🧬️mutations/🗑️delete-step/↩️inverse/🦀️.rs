//! ↩️ `delete-step` — undo re-creates the step (with its captured blocks) at its BASE-state index;
//! missing id ⇒ `Vec::new()`.

use crate::standards::v1::subsets::any::schema::mutations::delete_step::mutation::DeleteStep;
use crate::mutations::create_step::mutation::CreateStep;
use crate::{forms_steps, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_delete_step<Op>(payload: &DeleteStep, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_step::mutation::CreateStep>,
{
    Ok((|| {
    let steps = forms_steps(base);
    match steps.iter().position(|step| step.id == payload.id) {
        Some(index) => vec![Op::from(CreateStep { step: steps[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
