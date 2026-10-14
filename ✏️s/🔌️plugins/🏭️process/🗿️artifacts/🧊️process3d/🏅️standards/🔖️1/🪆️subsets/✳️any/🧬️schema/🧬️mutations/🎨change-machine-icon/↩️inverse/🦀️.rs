//! ↩️ `change-machine-icon` inverse — reconstructs the pre-change icon from BASE state; a machine
//! already absent from `base` has nothing to undo.

use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::change_machine_icon::ChangeMachineIcon>>(payload: &super::ChangeMachineIcon, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.workshop.machines.iter().find(|machine| machine.id == payload.id).map(|machine| vec![Op::from(super::ChangeMachineIcon { id: payload.id.clone(), new_icon_id: machine.icon_id.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
