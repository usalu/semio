//! ↩️ `change-step-enabled` inverse — reconstructs the pre-change flag from BASE state; a step
//! already absent from `base` has nothing to undo.

use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::change_step_enabled::ChangeStepEnabled>>(payload: &super::ChangeStepEnabled, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.step_payloads.iter().find(|step| step.id == payload.id).map(|step| vec![Op::from(super::ChangeStepEnabled { id: payload.id.clone(), new_enabled: step.enabled })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
