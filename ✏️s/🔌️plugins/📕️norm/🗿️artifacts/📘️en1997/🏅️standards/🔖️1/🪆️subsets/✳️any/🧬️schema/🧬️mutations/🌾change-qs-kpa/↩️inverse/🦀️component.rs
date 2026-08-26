//! ↩️ `change-qs-kpa` inverse — restores the pre-change `q_s_kpa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_q_s_kpa::mutation::ChangeQSKpa;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeQSKpa, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeQSKpa(ChangeQSKpa { new_q_s_kpa: base.q_s_kpa.clone() })]
}
//#endregion 🔖️Inverse
