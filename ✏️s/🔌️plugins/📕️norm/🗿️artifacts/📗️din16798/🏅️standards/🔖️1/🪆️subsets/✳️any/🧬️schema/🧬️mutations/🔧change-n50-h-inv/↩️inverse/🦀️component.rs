//! ↩️ `change-n50-h-inv` inverse — restores the pre-change `n50_h_inv` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_n50_h_inv::mutation::ChangeN50HInv;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeN50HInv, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeN50HInv(ChangeN50HInv { new_n50_h_inv: base.n50_h_inv.clone() })]
}
//#endregion 🔖️Inverse
