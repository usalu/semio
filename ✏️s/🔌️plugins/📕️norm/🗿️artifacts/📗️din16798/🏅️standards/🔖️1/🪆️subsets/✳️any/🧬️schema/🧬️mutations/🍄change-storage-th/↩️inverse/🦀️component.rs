//! ↩️ `change-storage-th` inverse — restores the pre-change `storage_t_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_storage_t_h::mutation::ChangeStorageTH;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeStorageTH, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeStorageTH(ChangeStorageTH { new_storage_t_h: base.storage_t_h.clone() })]
}
//#endregion 🔖️Inverse
