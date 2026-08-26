//! ↩️ `change-foundation-h-rd-kn` inverse — restores the pre-change `foundation_h_rd_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_foundation_h_rd_kn::mutation::ChangeFoundationHRdKn;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFoundationHRdKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeFoundationHRdKn(ChangeFoundationHRdKn { new_foundation_h_rd_kn: base.foundation_h_rd_kn.clone() })]
}
//#endregion 🔖️Inverse
