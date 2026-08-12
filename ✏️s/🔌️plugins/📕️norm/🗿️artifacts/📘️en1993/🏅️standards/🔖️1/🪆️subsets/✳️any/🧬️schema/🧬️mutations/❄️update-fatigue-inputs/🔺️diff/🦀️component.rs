//! 🔺️ `update-fatigue-inputs` — sparse diff construction.

use super::mutation::UpdateFatigueInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateFatigueInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        delta_sigma_mpa: Some(payload.new_delta_sigma_mpa),
        fatigue_category: Some(payload.new_fatigue_category),
        fatigue_method: Some(payload.new_fatigue_method.clone()),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
