//! ↩️ `replace-step-measure` inverse — reconstructs the pre-replace measure from BASE state; a
//! step already absent from `base` has nothing to undo.

use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_step_measure::ReplaceStepMeasure>>(payload: &super::ReplaceStepMeasure, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.step_payloads.iter().find(|step| step.id == payload.id).map(|step| vec![Op::from(super::ReplaceStepMeasure { id: payload.id.clone(), new_measure: step.measure.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
