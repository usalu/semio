//! ↩️ `replace-machine-capabilities` inverse — reconstructs the pre-replace capabilities list from
//! BASE state; a machine already absent from `base` has nothing to undo.

use crate::artifacts::process3d::mutations::replace_machine_capabilities::mutation::ReplaceMachineCapabilities;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ReplaceMachineCapabilities, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.workshop
        .machines
        .iter()
        .find(|machine| machine.id == payload.id)
        .map(|machine| vec![Process3dMutation::ReplaceMachineCapabilities(ReplaceMachineCapabilities { id: payload.id.clone(), new_capabilities: machine.capabilities.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
