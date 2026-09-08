//! ↩️ `change-liquid-f-ct-eff-mpa` inverse — restores the pre-change `liquid_f_ct_eff_mpa` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_liquid_f_ct_eff_mpa::ChangeLiquidFCtEffMpa;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeLiquidFCtEffMpa, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeLiquidFCtEffMpa(ChangeLiquidFCtEffMpa { new_liquid_f_ct_eff_mpa: base.liquid_f_ct_eff_mpa })]
}
//#endregion 🔖️Inverse
