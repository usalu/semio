//! 🔺️ `change-system-type` sparse diff construction — writes only `Din16798Diff.system_type` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_system_type::mutation::ChangeSystemType;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSystemType, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { system_type: Some(payload.new_system_type.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
