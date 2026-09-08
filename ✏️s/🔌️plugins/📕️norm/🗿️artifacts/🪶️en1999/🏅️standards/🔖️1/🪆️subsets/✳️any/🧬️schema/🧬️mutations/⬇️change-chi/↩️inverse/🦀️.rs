//! ↩️ `change-chi` inverse — restores the pre-change `chi` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_chi::ChangeChi;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeChi, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeChi(ChangeChi { new_chi: base.chi })]
}
//#endregion 🔖️Inverse
