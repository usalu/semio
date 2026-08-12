//! ↩️ `change-h-ed-kn` inverse — restores the pre-change `h_ed_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_h_ed_kn::mutation::ChangeHEdKn;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHEdKn, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeHEdKn(ChangeHEdKn { new_h_ed_kn: base.h_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
