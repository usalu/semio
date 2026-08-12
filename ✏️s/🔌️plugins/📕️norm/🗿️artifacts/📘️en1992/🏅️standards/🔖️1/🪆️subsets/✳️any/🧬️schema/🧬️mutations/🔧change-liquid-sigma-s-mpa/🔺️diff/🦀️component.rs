//! 🔺️ `change-liquid-sigma-s-mpa` sparse diff construction — writes only `En1992Diff.liquid_sigma_s_mpa` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_liquid_sigma_s_mpa::mutation::ChangeLiquidSigmaSMpa;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLiquidSigmaSMpa, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { liquid_sigma_s_mpa: Some(payload.new_liquid_sigma_s_mpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
