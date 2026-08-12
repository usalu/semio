//! 🔺️ `change-comfort-category` sparse diff construction — writes only `Din16798Diff.comfort_category` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_comfort_category::mutation::ChangeComfortCategory;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeComfortCategory, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { comfort_category: Some(payload.new_comfort_category.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
