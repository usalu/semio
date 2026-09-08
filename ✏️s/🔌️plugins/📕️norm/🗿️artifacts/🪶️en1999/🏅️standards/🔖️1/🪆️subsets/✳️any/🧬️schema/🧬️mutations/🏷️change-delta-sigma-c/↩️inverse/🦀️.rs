//! ↩️ `change-delta-sigma-c` inverse — restores the pre-change `delta_sigma_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_delta_sigma_c::ChangeDeltaSigmaC;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDeltaSigmaC, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeDeltaSigmaC(ChangeDeltaSigmaC { new_delta_sigma_c: base.delta_sigma_c })]
}
//#endregion 🔖️Inverse
