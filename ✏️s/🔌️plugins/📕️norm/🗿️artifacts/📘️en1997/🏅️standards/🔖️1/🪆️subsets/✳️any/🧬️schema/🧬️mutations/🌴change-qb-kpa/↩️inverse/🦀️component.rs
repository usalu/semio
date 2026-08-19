//! ↩️ `change-qb-kpa` inverse — restores the pre-change `q_b_kpa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_q_b_kpa::mutation::ChangeQBKpa;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeQBKpa, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeQBKpa(ChangeQBKpa { new_q_b_kpa: base.q_b_kpa.clone() })]
}
//#endregion 🔖️Inverse
