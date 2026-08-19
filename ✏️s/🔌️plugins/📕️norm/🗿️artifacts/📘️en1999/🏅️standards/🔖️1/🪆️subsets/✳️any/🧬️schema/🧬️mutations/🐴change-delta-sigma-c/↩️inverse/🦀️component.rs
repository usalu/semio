//! ↩️ `change-delta-sigma-c` inverse — restores the pre-change `delta_sigma_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_delta_sigma_c::mutation::ChangeDeltaSigmaC;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeDeltaSigmaC, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeDeltaSigmaC(ChangeDeltaSigmaC { new_delta_sigma_c: base.delta_sigma_c.clone() })]
}
//#endregion 🔖️Inverse
