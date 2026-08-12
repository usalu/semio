//! 🔺️ `change-rh-percent` sparse diff construction — writes only `Din16798Diff.rh_percent` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_rh_percent::mutation::ChangeRhPercent;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRhPercent, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { rh_percent: Some(payload.new_rh_percent.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
