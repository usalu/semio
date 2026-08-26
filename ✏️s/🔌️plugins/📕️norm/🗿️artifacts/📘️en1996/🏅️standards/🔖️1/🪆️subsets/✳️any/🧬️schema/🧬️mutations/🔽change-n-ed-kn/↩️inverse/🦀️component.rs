//! ↩️ `change-n-ed-kn` inverse — restores the pre-change `n_ed_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_n_ed_kn::mutation::ChangeNEdKn;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeNEdKn, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeNEdKn(ChangeNEdKn { new_n_ed_kn: base.n_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
