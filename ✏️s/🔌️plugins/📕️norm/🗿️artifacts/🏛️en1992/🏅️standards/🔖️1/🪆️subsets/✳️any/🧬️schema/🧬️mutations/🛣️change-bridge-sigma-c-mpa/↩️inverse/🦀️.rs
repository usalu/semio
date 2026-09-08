//! ↩️ `change-bridge-sigma-c-mpa` inverse — restores the pre-change `bridge_sigma_c_mpa` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_bridge_sigma_c_mpa::ChangeBridgeSigmaCMpa;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBridgeSigmaCMpa, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeBridgeSigmaCMpa(ChangeBridgeSigmaCMpa { new_bridge_sigma_c_mpa: base.bridge_sigma_c_mpa })]
}
//#endregion 🔖️Inverse
