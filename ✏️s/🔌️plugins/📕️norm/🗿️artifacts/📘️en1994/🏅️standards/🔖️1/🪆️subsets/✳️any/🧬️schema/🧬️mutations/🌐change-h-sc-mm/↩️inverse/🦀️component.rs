//! ↩️ `change-h-sc-mm` — undo restores BASE's h_sc_mm.

use super::mutation::ChangeHScMm;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeHScMm, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeHScMm(ChangeHScMm { new_h_sc_mm: base.h_sc_mm })]
}
//#endregion 🔖️Inverse
