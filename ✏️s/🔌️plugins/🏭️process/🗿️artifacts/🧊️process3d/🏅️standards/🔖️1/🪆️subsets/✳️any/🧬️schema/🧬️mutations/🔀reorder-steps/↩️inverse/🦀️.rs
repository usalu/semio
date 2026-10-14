//! ↩️ `reorder-steps` inverse — reconstructs the pre-move position from BASE state; a step
//! already absent from `base` has nothing to undo.

use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::reorder_steps::ReorderSteps>>(payload: &super::ReorderSteps, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(from) = base.step_payloads.iter().position(|step| step.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(super::ReorderSteps { id: payload.id.clone(), to_index: from })]

    })())
}
//#endregion 🔖️Inverse
