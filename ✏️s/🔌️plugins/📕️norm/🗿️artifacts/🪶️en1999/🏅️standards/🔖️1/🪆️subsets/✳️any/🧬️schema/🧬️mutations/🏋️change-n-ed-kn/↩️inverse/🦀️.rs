//! ↩️ `change-n-ed-kn` inverse — restores the pre-change `n_ed_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_n_ed_kn::ChangeNEdKn;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeNEdKn, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeNEdKn(ChangeNEdKn { new_n_ed_kn: base.n_ed_kn })]
}
//#endregion 🔖️Inverse
