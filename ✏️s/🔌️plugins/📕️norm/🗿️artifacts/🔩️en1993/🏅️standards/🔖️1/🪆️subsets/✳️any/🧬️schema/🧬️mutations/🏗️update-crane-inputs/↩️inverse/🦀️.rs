//! ↩️ `update-crane-inputs` — undo restores BASE's crane inputs.

use super::UpdateCraneInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateCraneInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateCraneInputs(UpdateCraneInputs {
        new_crane_f_z_ed_kn: base.crane_f_z_ed_kn,
        new_crane_wheel_contact_length_mm: base.crane_wheel_contact_length_mm,
        new_crane_dispersion_mm: base.crane_dispersion_mm,
        new_crane_t_w_mm: base.crane_t_w_mm,
    })]
}
//#endregion 🔖️Inverse
