//! ↩️ `update-weld-inputs` — undo restores BASE's weld inputs.

use super::mutation::UpdateWeldInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateWeldInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateWeldInputs(UpdateWeldInputs {
        new_weld_a_mm: base.weld_a_mm,
        new_weld_l_mm: base.weld_l_mm,
        new_weld_f_u_mpa: base.weld_f_u_mpa,
        new_weld_steel_grade: base.weld_steel_grade.clone(),
        new_weld_f_ed_kn: base.weld_f_ed_kn,
    })]
}
//#endregion 🔖️Inverse
