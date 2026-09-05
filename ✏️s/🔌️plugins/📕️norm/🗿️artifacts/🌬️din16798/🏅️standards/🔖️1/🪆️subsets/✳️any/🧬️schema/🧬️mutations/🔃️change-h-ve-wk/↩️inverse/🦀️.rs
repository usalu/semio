//! ↩️ `change-h-ve-wk` inverse — restores the pre-change `h_ve_w_k` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_h_ve_w_k::ChangeHVeWK;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHVeWK, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHVeWK(ChangeHVeWK { new_h_ve_w_k: base.h_ve_w_k.clone() })]
}
//#endregion 🔖️Inverse
