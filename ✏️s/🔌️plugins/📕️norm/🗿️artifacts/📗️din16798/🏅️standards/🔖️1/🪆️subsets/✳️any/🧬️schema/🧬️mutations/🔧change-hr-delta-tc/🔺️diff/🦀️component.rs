//! 🔺️ `change-hr-delta-tc` sparse diff construction — writes only `Din16798Diff.hr_delta_t_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_hr_delta_t_c::mutation::ChangeHrDeltaTC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHrDeltaTC, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { hr_delta_t_c: Some(payload.new_hr_delta_t_c.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
