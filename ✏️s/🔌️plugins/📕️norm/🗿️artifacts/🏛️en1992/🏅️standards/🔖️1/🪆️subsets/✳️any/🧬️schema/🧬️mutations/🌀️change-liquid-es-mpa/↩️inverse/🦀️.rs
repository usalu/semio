//! ↩️ `change-liquid-es-mpa` inverse — restores the pre-change `liquid_e_s_mpa` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_liquid_e_s_mpa::ChangeLiquidESMpa;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeLiquidESMpa, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeLiquidESMpa(ChangeLiquidESMpa { new_liquid_e_s_mpa: base.liquid_e_s_mpa })]
}
//#endregion 🔖️Inverse
