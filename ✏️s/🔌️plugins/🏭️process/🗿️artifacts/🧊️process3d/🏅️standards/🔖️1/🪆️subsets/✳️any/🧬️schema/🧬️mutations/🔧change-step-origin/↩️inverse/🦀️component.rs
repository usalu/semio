//! ↩️ `change-step-origin` inverse — reconstructs the pre-change `origin` value from BASE state;
//! a step already absent from `base` has nothing to undo.

use crate::artifacts::process3d::mutations::change_step_origin::mutation::ChangeStepOrigin;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeStepOrigin, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.steps
        .iter()
        .find(|step| step.id == payload.id)
        .map(|step| vec![Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: payload.id.clone(), new_origin: step.origin.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
