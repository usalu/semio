//! ↩️ `change-h-tr-wk` inverse — restores the pre-change `h_tr_w_k` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_h_tr_w_k::ChangeHTrWK;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHTrWK, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHTrWK(ChangeHTrWK { new_h_tr_w_k: base.h_tr_w_k.clone() })]
}
//#endregion 🔖️Inverse
