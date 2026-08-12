//! 🔺️ `change-bedrooms` sparse diff construction — writes only `Din16798Diff.bedrooms` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_bedrooms::mutation::ChangeBedrooms;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBedrooms, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { bedrooms: Some(payload.new_bedrooms.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
