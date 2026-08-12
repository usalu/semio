//! 🔺️ `change-storage-allowance-kwh` sparse diff construction — writes only `Din16798Diff.storage_allowance_kwh` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_storage_allowance_kwh::mutation::ChangeStorageAllowanceKwh;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStorageAllowanceKwh, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { storage_allowance_kwh: Some(payload.new_storage_allowance_kwh.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
