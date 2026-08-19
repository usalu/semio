//! ↩️ `change-n-ed-kn` inverse — restores the pre-change `n_ed_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_n_ed_kn::mutation::ChangeNEdKn;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeNEdKn, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeNEdKn(ChangeNEdKn { new_n_ed_kn: base.n_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
