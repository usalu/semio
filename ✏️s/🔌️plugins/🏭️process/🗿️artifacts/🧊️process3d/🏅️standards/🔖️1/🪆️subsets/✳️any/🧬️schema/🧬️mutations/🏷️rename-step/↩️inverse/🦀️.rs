//! ↩️ `rename-step` inverse — reconstructs the pre-rename label from BASE state; a step already
//! absent from `base` has nothing to undo.

use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::rename_step::RenameStep>>(payload: &super::RenameStep, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.step_payloads.iter().find(|step| step.id == payload.id).map(|step| vec![Op::from(super::RenameStep { id: payload.id.clone(), new_label: step.label.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
