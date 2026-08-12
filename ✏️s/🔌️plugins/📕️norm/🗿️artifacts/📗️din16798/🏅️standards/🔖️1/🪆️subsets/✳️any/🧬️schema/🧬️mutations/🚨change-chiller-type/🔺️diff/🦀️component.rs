//! 🔺️ `change-chiller-type` sparse diff construction — writes only `Din16798Diff.chiller_type` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_chiller_type::mutation::ChangeChillerType;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeChillerType, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { chiller_type: Some(payload.new_chiller_type.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
