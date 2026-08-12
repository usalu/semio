//! ↩️ `change-delta-sigma-ed` inverse — restores the pre-change `delta_sigma_ed` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_delta_sigma_ed::mutation::ChangeDeltaSigmaEd;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDeltaSigmaEd, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeDeltaSigmaEd(ChangeDeltaSigmaEd { new_delta_sigma_ed: base.delta_sigma_ed.clone() })]
}
//#endregion 🔖️Inverse
