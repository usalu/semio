//! 🔺️ `change-hr-savings-reference-kwh` sparse diff construction — writes only `Din16798Diff.hr_savings_reference_kwh` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_hr_savings_reference_kwh::mutation::ChangeHrSavingsReferenceKwh;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHrSavingsReferenceKwh, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { hr_savings_reference_kwh: Some(payload.new_hr_savings_reference_kwh.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
