//! ↩️ `change-storage-allowance-kwh` inverse — restores the pre-change `storage_allowance_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_storage_allowance_kwh::ChangeStorageAllowanceKwh;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeStorageAllowanceKwh, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeStorageAllowanceKwh(ChangeStorageAllowanceKwh { new_storage_allowance_kwh: base.storage_allowance_kwh })]
}
//#endregion 🔖️Inverse
