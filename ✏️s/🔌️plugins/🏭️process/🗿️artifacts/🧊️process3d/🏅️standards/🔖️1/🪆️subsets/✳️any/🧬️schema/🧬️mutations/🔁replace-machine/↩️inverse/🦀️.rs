//! ↩️ `replace-machine-capabilities` inverse — reconstructs the pre-replace capabilities list from
//! BASE state; a machine already absent from `base` has nothing to undo.

use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::replace_machine_capabilities::ReplaceMachineCapabilities>>(payload: &super::ReplaceMachineCapabilities, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.workshop
        .machines
        .iter()
        .find(|machine| machine.id == payload.id)
        .map(|machine| vec![Op::from(super::ReplaceMachineCapabilities { id: payload.id.clone(), new_capabilities: machine.capabilities.clone() })])
        .unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
