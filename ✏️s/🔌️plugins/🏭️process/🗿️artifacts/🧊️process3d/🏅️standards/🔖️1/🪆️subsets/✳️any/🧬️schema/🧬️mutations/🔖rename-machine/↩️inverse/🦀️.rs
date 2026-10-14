//! ↩️ `rename-machine` inverse — reconstructs the pre-rename label from BASE state; a machine
//! already absent from `base` has nothing to undo.

use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_machine::RenameMachine>>(payload: &super::RenameMachine, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.workshop.machines.iter().find(|machine| machine.id == payload.id).map(|machine| vec![Op::from(super::RenameMachine { id: payload.id.clone(), new_label: machine.label.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
