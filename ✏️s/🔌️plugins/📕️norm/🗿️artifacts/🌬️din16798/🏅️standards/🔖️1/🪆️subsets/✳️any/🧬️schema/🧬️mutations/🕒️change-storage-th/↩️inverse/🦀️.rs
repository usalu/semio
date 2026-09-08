//! ↩️ `change-storage-th` inverse — restores the pre-change `storage_t_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_storage_t_h::ChangeStorageTH;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeStorageTH, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeStorageTH(ChangeStorageTH { new_storage_t_h: base.storage_t_h })]
}
//#endregion 🔖️Inverse
