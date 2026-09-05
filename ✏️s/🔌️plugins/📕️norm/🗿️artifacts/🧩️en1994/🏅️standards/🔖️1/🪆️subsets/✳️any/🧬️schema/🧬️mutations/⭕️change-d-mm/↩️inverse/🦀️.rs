//! ↩️ `change-d-mm` — undo restores BASE's d_mm.

use super::ChangeDMm;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDMm, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeDMm(ChangeDMm { new_d_mm: base.d_mm })]
}
//#endregion 🔖️Inverse
