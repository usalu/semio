//! ↩️ `change-insulation-thickness-mm` — undo restores BASE's insulation_thickness_mm.

use super::mutation::ChangeInsulationThicknessMm;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeInsulationThicknessMm, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeInsulationThicknessMm(ChangeInsulationThicknessMm { new_insulation_thickness_mm: base.insulation_thickness_mm })]
}
//#endregion 🔖️Inverse
