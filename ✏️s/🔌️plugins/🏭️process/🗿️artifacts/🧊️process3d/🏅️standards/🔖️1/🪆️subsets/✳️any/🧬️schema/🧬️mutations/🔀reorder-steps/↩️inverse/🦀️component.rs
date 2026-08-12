//! ↩️ `reorder-steps` inverse — moves the step back to its BASE-state position; a step already
//! absent from `base` has nothing to undo (addressing convention #5, `📓️taxonomy.md`).

use crate::artifacts::process3d::mutations::reorder_steps::mutation::ReorderSteps;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReorderSteps, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.steps
        .iter()
        .position(|step| step.id == payload.id)
        .map(|from_index| vec![Process3dMutation::ReorderSteps(ReorderSteps { id: payload.id.clone(), to_index: from_index })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
