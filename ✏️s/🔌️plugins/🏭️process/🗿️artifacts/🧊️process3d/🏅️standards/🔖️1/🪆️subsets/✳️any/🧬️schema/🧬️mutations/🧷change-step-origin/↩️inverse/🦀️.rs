//! ↩️ `change-step-origin` inverse — reconstructs the pre-change origin from BASE state; a step
//! already absent from `base` has nothing to undo.

use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::change_step_origin::ChangeStepOrigin>>(payload: &super::ChangeStepOrigin, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.step_payloads.iter().find(|step| step.id == payload.id).map(|step| vec![Op::from(super::ChangeStepOrigin { id: payload.id.clone(), new_origin: step.origin.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
