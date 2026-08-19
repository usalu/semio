//! ↩️ `change-n-pile-ed-kn` inverse — restores the pre-change `n_pile_ed_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_n_pile_ed_kn::mutation::ChangeNPileEdKn;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeNPileEdKn, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeNPileEdKn(ChangeNPileEdKn { new_n_pile_ed_kn: base.n_pile_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
