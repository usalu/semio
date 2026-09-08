//! ↩️ `change-h-tr-wk` inverse — restores the pre-change `h_tr_w_k` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_h_tr_w_k::ChangeHTrWK;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHTrWK, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHTrWK(ChangeHTrWK { new_h_tr_w_k: base.h_tr_w_k })]
}
//#endregion 🔖️Inverse
