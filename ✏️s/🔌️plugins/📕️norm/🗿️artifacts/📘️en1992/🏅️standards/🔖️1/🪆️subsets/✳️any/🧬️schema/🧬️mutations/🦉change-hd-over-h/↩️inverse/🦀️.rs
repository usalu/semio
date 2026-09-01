//! ↩️ `change-hd-over-h` inverse — restores the pre-change `hd_over_h` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_hd_over_h::ChangeHdOverH;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHdOverH, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeHdOverH(ChangeHdOverH { new_hd_over_h: base.hd_over_h.clone() })]
}
//#endregion 🔖️Inverse
