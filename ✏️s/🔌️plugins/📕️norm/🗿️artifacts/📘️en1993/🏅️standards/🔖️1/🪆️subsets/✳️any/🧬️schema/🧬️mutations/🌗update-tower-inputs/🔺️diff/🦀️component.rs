//! 🔺️ `update-tower-inputs` — sparse diff construction.

use super::mutation::UpdateTowerInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateTowerInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff { tower_wind_factor: Some(payload.new_tower_wind_factor), tower_n_ed_kn: Some(payload.new_tower_n_ed_kn), ..Default::default() }
}
//#endregion 🔖️Diff
