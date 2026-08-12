//! 🔺️ `change-liquid-rho-p-eff` sparse diff construction — writes only `En1992Diff.liquid_rho_p_eff` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_liquid_rho_p_eff::mutation::ChangeLiquidRhoPEff;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLiquidRhoPEff, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { liquid_rho_p_eff: Some(payload.new_liquid_rho_p_eff.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
