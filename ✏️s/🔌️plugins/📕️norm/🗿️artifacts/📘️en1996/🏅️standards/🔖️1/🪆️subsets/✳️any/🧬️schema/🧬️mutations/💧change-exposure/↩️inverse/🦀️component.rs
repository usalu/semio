//! ↩️ `change-exposure` inverse — restores the pre-change `exposure` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_exposure::mutation::ChangeExposure;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeExposure, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeExposure(ChangeExposure { new_exposure: base.exposure.clone() })]
}
//#endregion 🔖️Inverse
