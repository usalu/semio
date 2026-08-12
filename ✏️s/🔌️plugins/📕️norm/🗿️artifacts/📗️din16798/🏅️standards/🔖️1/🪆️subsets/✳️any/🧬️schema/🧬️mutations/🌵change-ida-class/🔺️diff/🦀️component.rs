//! 🔺️ `change-ida-class` sparse diff construction — writes only `Din16798Diff.ida_class` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_ida_class::mutation::ChangeIdaClass;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeIdaClass, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { ida_class: Some(payload.new_ida_class.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
