//! ↩️ `change-p-kn` inverse — restores the pre-change `p_kn` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_p_kn::mutation::ChangePKn;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangePKn, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangePKn(ChangePKn { new_p_kn: base.p_kn.clone() })]
}
//#endregion 🔖️Inverse
