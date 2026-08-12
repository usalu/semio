//! 🔺️ `change-hr-cp-j-kgk` sparse diff construction — writes only `Din16798Diff.hr_cp_j_kgk` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_hr_cp_j_kgk::mutation::ChangeHrCpJKgk;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHrCpJKgk, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { hr_cp_j_kgk: Some(payload.new_hr_cp_j_kgk.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
