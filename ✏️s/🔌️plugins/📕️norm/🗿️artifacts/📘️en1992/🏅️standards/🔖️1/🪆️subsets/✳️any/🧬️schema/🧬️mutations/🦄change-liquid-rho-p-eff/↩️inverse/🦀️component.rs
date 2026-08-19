//! ↩️ `change-liquid-rho-p-eff` inverse — restores the pre-change `liquid_rho_p_eff` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_liquid_rho_p_eff::mutation::ChangeLiquidRhoPEff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeLiquidRhoPEff, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeLiquidRhoPEff(ChangeLiquidRhoPEff { new_liquid_rho_p_eff: base.liquid_rho_p_eff.clone() })]
}
//#endregion 🔖️Inverse
