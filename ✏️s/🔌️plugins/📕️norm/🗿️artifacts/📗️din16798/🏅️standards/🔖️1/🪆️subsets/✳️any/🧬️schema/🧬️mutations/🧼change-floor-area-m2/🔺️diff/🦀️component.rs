//! 🔺️ `change-floor-area-m2` sparse diff construction — writes only `Din16798Diff.floor_area_m2` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_floor_area_m2::mutation::ChangeFloorAreaM2;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFloorAreaM2, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { floor_area_m2: Some(payload.new_floor_area_m2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
