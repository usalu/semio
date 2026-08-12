//! 🔺️ `change-cooling-utilization-factor` sparse diff construction — writes only `Din16798Diff.cooling_utilization_factor` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_cooling_utilization_factor::mutation::ChangeCoolingUtilizationFactor;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCoolingUtilizationFactor, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { cooling_utilization_factor: Some(payload.new_cooling_utilization_factor.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
