//! ↩️ `change-hv` inverse — restores the pre-change `h_v` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_h_v::ChangeHV;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHV, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeHV(ChangeHV { new_h_v: base.h_v })]
}
//#endregion 🔖️Inverse
