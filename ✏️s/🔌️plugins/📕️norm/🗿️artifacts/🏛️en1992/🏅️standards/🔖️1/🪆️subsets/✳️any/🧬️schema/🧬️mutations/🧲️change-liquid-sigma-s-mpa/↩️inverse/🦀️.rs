//! ↩️ `change-liquid-sigma-s-mpa` inverse — restores the pre-change `liquid_sigma_s_mpa` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_liquid_sigma_s_mpa::ChangeLiquidSigmaSMpa;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeLiquidSigmaSMpa, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeLiquidSigmaSMpa(ChangeLiquidSigmaSMpa { new_liquid_sigma_s_mpa: base.liquid_sigma_s_mpa })]
}
//#endregion 🔖️Inverse
