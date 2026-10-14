//! ↩️ `change-correction-as-of` — undo restores BASE's edition.

use crate::change_correction_as_of::ChangeCorrectionAsOf;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeCorrectionAsOf, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_correction_as_of::ChangeCorrectionAsOf> {
    Ok((|| {
    vec![Op::from(ChangeCorrectionAsOf { new_correction_as_of: base.correction_as_of })]

    })())
}
//#endregion 🔖️Inverse
