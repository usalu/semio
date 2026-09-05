//! ↩️ `change-h-ed-kn` inverse — restores the pre-change `h_ed_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_h_ed_kn::ChangeHEdKn;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHEdKn, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeHEdKn(ChangeHEdKn { new_h_ed_kn: base.h_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
