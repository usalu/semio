//! ↩️ `change-correction-as-of` — undo restores BASE's edition.

use super::mutation::ChangeCorrectionAsOf;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeCorrectionAsOf, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    vec![Vdi3805Mutation::ChangeCorrectionAsOf(ChangeCorrectionAsOf { new_correction_as_of: base.correction_as_of })]
}
//#endregion 🔖️Inverse
