//! 🔺️ `update-weld-inputs` — sparse diff construction.

use super::mutation::UpdateWeldInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateWeldInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        weld_a_mm: Some(payload.new_weld_a_mm),
        weld_l_mm: Some(payload.new_weld_l_mm),
        weld_f_u_mpa: Some(payload.new_weld_f_u_mpa),
        weld_steel_grade: Some(payload.new_weld_steel_grade.clone()),
        weld_f_ed_kn: Some(payload.new_weld_f_ed_kn),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
