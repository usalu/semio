//! 🔺️ `change-use-class` sparse diff construction — writes only `Din18599Diff.use_class` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_use_class::mutation::ChangeUseClass;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeUseClass, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { use_class: Some(payload.new_use_class.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
