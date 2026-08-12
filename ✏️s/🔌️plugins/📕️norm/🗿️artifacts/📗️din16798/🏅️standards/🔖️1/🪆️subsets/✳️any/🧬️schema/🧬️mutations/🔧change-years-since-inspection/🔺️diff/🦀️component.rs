//! 🔺️ `change-years-since-inspection` sparse diff construction — writes only `Din16798Diff.years_since_inspection` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_years_since_inspection::mutation::ChangeYearsSinceInspection;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeYearsSinceInspection, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { years_since_inspection: Some(payload.new_years_since_inspection.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
