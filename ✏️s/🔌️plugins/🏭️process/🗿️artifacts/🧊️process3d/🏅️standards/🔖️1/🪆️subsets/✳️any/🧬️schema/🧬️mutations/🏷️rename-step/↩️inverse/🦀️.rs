//! ↩️ `rename-step` inverse — reconstructs the pre-rename label from BASE state; a step already
//! absent from `base` has nothing to undo.

use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RenameStep, base: &Process3dSnapshot) -> Result<Vec<Process3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.step_payloads.iter().find(|step| step.id == payload.id).map(|step| vec![Process3dMutation::RenameStep(super::RenameStep { id: payload.id.clone(), new_label: step.label.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
