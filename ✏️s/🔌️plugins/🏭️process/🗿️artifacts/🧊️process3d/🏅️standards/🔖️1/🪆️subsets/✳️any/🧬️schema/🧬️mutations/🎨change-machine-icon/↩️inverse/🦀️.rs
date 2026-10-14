//! ↩️ `change-machine-icon` inverse — reconstructs the pre-change icon from BASE state; a machine
//! already absent from `base` has nothing to undo.

use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::change_machine_icon::ChangeMachineIcon>>(payload: &super::ChangeMachineIcon, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.workshop.machines.iter().find(|machine| machine.id == payload.id).map(|machine| vec![Op::from(super::ChangeMachineIcon { id: payload.id.clone(), new_icon_id: machine.icon_id.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
