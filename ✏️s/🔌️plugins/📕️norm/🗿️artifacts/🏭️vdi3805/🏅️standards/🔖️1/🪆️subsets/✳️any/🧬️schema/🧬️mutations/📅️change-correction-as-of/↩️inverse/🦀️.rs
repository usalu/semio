//! ↩️ `change-correction-as-of` — undo restores BASE's edition.

use super::ChangeCorrectionAsOf;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeCorrectionAsOf, base: &Vdi3805Snapshot) -> Result<Vec<Vdi3805Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Vdi3805Mutation::ChangeCorrectionAsOf(ChangeCorrectionAsOf { new_correction_as_of: base.correction_as_of })]

    })())
}
//#endregion 🔖️Inverse
