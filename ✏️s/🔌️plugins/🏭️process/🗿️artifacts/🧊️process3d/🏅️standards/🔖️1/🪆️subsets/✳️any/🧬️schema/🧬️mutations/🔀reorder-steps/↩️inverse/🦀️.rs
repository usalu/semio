//! ↩️ `reorder-steps` inverse — reconstructs the pre-move position from BASE state; a step
//! already absent from `base` has nothing to undo.

use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ReorderSteps, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    let Some(from) = base.step_payloads.iter().position(|step| step.id == payload.id) else {
        return Vec::new();
    };
    vec![Process3dMutation::ReorderSteps(super::ReorderSteps { id: payload.id.clone(), to_index: from })]
}
//#endregion 🔖️Inverse
