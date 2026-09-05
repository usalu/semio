//! ↩️ `change-liquid-sr-max-mm` inverse — restores the pre-change `liquid_s_r_max_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_liquid_s_r_max_mm::ChangeLiquidSRMaxMm;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeLiquidSRMaxMm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeLiquidSRMaxMm(ChangeLiquidSRMaxMm { new_liquid_s_r_max_mm: base.liquid_s_r_max_mm.clone() })]
}
//#endregion 🔖️Inverse
