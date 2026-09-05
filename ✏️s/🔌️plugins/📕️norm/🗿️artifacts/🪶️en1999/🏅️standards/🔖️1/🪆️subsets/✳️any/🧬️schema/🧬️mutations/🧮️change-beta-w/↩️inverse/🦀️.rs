//! ↩️ `change-beta-w` inverse — restores the pre-change `beta_w` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_beta_w::ChangeBetaW;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBetaW, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeBetaW(ChangeBetaW { new_beta_w: base.beta_w.clone() })]
}
//#endregion 🔖️Inverse
