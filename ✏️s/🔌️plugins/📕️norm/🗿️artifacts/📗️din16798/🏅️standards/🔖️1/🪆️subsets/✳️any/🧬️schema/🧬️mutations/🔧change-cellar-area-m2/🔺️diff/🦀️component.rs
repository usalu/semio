//! 🔺️ `change-cellar-area-m2` sparse diff construction — writes only `Din16798Diff.cellar_area_m2` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_cellar_area_m2::mutation::ChangeCellarAreaM2;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCellarAreaM2, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { cellar_area_m2: Some(payload.new_cellar_area_m2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
