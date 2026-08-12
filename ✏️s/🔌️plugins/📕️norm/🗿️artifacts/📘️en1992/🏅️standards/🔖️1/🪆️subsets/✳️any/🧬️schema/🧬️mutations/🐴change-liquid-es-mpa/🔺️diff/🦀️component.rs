//! 🔺️ `change-liquid-es-mpa` sparse diff construction — writes only `En1992Diff.liquid_e_s_mpa` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_liquid_e_s_mpa::mutation::ChangeLiquidESMpa;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLiquidESMpa, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { liquid_e_s_mpa: Some(payload.new_liquid_e_s_mpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
