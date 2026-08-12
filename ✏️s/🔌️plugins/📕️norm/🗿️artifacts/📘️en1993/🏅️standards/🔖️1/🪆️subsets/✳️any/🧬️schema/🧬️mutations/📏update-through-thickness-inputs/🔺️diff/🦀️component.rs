//! 🔺️ `update-through-thickness-inputs` — sparse diff construction.

use super::mutation::UpdateThroughThicknessInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateThroughThicknessInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        t10_steel_subgrade: Some(payload.new_t10_steel_subgrade.clone()),
        t10_actual_thickness_mm: Some(payload.new_t10_actual_thickness_mm),
        t10_t_ed_c: Some(payload.new_t10_t_ed_c),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
