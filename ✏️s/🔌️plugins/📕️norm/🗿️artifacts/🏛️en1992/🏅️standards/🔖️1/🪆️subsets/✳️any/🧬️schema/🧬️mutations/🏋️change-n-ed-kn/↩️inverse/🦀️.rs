//! ↩️ `change-n-ed-kn` inverse — restores the pre-change `n_ed_kn` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_n_ed_kn::ChangeNEdKn;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeNEdKn, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeNEdKn(ChangeNEdKn { new_n_ed_kn: base.n_ed_kn })]
}
//#endregion 🔖️Inverse
