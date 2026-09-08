//! ↩️ `replace-step-measure` inverse — reconstructs the pre-replace measure from BASE state; a
//! step already absent from `base` has nothing to undo.

use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ReplaceStepMeasure, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.step_payloads.iter().find(|step| step.id == payload.id).map(|step| vec![Process3dMutation::ReplaceStepMeasure(super::ReplaceStepMeasure { id: payload.id.clone(), new_measure: step.measure.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
