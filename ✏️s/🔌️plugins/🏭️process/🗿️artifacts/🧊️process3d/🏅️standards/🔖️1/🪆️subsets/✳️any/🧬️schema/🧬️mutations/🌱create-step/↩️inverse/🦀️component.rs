//! ↩️ `create-step` inverse — undo of a create is always a `delete-step` by the created id (per
//! `📓️taxonomy.md`'s `create ↔ delete` pairing).

use crate::artifacts::process3d::mutations::delete_step::mutation::DeleteStep;
use crate::artifacts::process3d::mutations::create_step::mutation::CreateStep;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
/// ↩️ Undoing a create is deleting the same step back out, by its own id.
pub fn inverse(payload: &CreateStep, _base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    vec![Process3dMutation::DeleteStep(DeleteStep { id: payload.step.id.clone() })]
}
//#endregion 🔖️Inverse
