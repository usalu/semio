//! ↩️ `change-h-st-wk` inverse — restores the pre-change `h_st_w_k` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_h_st_w_k::ChangeHStWK;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHStWK, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHStWK(ChangeHStWK { new_h_st_w_k: base.h_st_w_k })]
}
//#endregion 🔖️Inverse
