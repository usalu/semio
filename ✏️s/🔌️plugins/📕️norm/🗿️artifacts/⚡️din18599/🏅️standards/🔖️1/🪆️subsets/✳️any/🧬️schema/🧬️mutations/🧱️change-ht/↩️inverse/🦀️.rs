//! ↩️ `change-ht` inverse — restores the pre-change `h_t` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_h_t::ChangeHT;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHT, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeHT(ChangeHT { new_h_t: base.h_t })]
}
//#endregion 🔖️Inverse
