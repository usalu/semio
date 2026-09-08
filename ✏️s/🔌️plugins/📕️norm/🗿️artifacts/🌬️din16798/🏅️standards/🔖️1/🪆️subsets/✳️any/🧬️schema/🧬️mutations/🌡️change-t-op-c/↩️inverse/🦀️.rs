//! ↩️ `change-t-op-c` inverse — restores the pre-change `t_op_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_t_op_c::ChangeTOpC;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeTOpC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeTOpC(ChangeTOpC { new_t_op_c: base.t_op_c })]
}
//#endregion 🔖️Inverse
