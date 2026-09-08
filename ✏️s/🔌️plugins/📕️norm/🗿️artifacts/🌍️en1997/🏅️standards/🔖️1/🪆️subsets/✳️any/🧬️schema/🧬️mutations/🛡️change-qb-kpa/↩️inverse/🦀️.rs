//! ↩️ `change-qb-kpa` inverse — restores the pre-change `q_b_kpa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_q_b_kpa::ChangeQBKpa;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeQBKpa, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeQBKpa(ChangeQBKpa { new_q_b_kpa: base.q_b_kpa })]
}
//#endregion 🔖️Inverse
