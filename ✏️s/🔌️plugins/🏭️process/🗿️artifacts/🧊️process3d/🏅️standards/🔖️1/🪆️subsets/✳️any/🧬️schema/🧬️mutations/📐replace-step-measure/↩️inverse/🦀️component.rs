//! ↩️ `replace-step-measure` inverse — reconstructs the pre-replace measure from BASE state; a
//! step already absent from `base` has nothing to undo.

use crate::artifacts::process3d::mutations::replace_step_measure::mutation::ReplaceStepMeasure;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceStepMeasure, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.steps
        .iter()
        .find(|step| step.id == payload.id)
        .map(|step| vec![Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: payload.id.clone(), new_measure: step.measure.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
