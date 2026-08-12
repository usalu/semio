//! 🔺️ `change-infiltration-allowance-m3-h` sparse diff construction — writes only `Din16798Diff.infiltration_allowance_m3_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_infiltration_allowance_m3_h::mutation::ChangeInfiltrationAllowanceM3H;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeInfiltrationAllowanceM3H, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { infiltration_allowance_m3_h: Some(payload.new_infiltration_allowance_m3_h.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
