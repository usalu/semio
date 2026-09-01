//! ↩️ `change-liquid-f-ct-eff-mpa` inverse — restores the pre-change `liquid_f_ct_eff_mpa` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_liquid_f_ct_eff_mpa::ChangeLiquidFCtEffMpa;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeLiquidFCtEffMpa, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeLiquidFCtEffMpa(ChangeLiquidFCtEffMpa { new_liquid_f_ct_eff_mpa: base.liquid_f_ct_eff_mpa.clone() })]
}
//#endregion 🔖️Inverse
