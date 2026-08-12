//! ↩️ `change-n-ed-kn` inverse — restores the pre-change `n_ed_kn` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_n_ed_kn::mutation::ChangeNEdKn;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeNEdKn, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeNEdKn(ChangeNEdKn { new_n_ed_kn: base.n_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
