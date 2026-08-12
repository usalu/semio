//! 🔺️ `change-storage-th` sparse diff construction — writes only `Din16798Diff.storage_t_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_storage_t_h::mutation::ChangeStorageTH;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStorageTH, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { storage_t_h: Some(payload.new_storage_t_h.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
