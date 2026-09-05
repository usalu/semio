//! ↩️ `change-chi` inverse — restores the pre-change `chi` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_chi::ChangeChi;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeChi, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeChi(ChangeChi { new_chi: base.chi.clone() })]
}
//#endregion 🔖️Inverse
