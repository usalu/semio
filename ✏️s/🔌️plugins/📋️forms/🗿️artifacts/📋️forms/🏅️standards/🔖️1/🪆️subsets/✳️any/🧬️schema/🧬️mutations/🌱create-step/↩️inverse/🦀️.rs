//! ↩️ `create-step` — undo is `delete-step`, unless `base` already had this id (then `create` was a
//! no-op and there's nothing to undo).

use crate::standards::v1::subsets::any::schema::mutations::create_step::mutation::CreateStep;
use crate::mutations::delete_step::mutation::DeleteStep;
use crate::{forms_steps, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_create_step<Op>(payload: &CreateStep, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_step::mutation::DeleteStep>,
{
    Ok((|| {
    if forms_steps(base).iter().any(|step| step.id == payload.step.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteStep { id: payload.step.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
