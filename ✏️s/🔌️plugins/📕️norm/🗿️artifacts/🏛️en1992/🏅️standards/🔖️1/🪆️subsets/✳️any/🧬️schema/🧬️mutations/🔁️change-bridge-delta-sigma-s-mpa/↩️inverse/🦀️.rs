//! ↩️ `change-bridge-delta-sigma-s-mpa` inverse — restores the pre-change `bridge_delta_sigma_s_mpa` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_bridge_delta_sigma_s_mpa::ChangeBridgeDeltaSigmaSMpa;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBridgeDeltaSigmaSMpa, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeBridgeDeltaSigmaSMpa(ChangeBridgeDeltaSigmaSMpa { new_bridge_delta_sigma_s_mpa: base.bridge_delta_sigma_s_mpa })]
}
//#endregion 🔖️Inverse
