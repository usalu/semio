//! ↩️ `delete-step` inverse — reconstructs a `create-step` from BASE state (original list position
//! + full payload); a step already absent from `base` has nothing to undo.

use crate::create_step::CreateStep;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_step::CreateStep>>(payload: &super::DeleteStep, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.step_payloads.iter().position(|step| step.id == payload.id).map(|index| vec![Op::from(CreateStep { index, step: base.step_payloads[index].clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
