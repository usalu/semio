//! 🔺️ `update-fire-inputs` — sparse diff construction.

use super::mutation::UpdateFireInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateFireInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        fire_thickness_mm: Some(payload.new_fire_thickness_mm),
        fire_rating: Some(payload.new_fire_rating.clone()),
        fire_massivity: Some(payload.new_fire_massivity),
        fire_mu_0: Some(payload.new_fire_mu_0),
        fire_design_temperature_c: Some(payload.new_fire_design_temperature_c),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
