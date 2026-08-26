//! ↩️ `change-v-ed-kn` inverse — restores the pre-change `v_ed_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_v_ed_kn::mutation::ChangeVEdKn;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeVEdKn, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeVEdKn(ChangeVEdKn { new_v_ed_kn: base.v_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
