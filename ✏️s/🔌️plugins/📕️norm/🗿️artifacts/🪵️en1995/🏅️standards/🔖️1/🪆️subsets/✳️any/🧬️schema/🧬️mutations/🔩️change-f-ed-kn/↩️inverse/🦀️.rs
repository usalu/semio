//! ↩️ `change-f-ed-kn` inverse — restores the pre-change `f_ed_kn` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_f_ed_kn::ChangeFEdKn;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFEdKn, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeFEdKn(ChangeFEdKn { new_f_ed_kn: base.f_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
