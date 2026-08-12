//! 🔺️ `update-crane-inputs` — sparse diff construction.

use super::mutation::UpdateCraneInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateCraneInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        crane_f_z_ed_kn: Some(payload.new_crane_f_z_ed_kn),
        crane_wheel_contact_length_mm: Some(payload.new_crane_wheel_contact_length_mm),
        crane_dispersion_mm: Some(payload.new_crane_dispersion_mm),
        crane_t_w_mm: Some(payload.new_crane_t_w_mm),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
