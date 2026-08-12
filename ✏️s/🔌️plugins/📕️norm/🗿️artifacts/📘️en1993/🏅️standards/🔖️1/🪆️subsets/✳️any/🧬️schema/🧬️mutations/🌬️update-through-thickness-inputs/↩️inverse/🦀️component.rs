//! ↩️ `update-through-thickness-inputs` — undo restores BASE's through thickness inputs.

use super::mutation::UpdateThroughThicknessInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateThroughThicknessInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateThroughThicknessInputs(UpdateThroughThicknessInputs {
        new_t10_steel_subgrade: base.t10_steel_subgrade.clone(),
        new_t10_actual_thickness_mm: base.t10_actual_thickness_mm,
        new_t10_t_ed_c: base.t10_t_ed_c,
    })]
}
//#endregion 🔖️Inverse
