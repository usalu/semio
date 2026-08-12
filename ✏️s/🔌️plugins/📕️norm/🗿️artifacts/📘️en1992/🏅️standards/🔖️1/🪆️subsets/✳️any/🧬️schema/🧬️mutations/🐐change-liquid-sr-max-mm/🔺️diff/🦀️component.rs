//! 🔺️ `change-liquid-sr-max-mm` sparse diff construction — writes only `En1992Diff.liquid_s_r_max_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_liquid_s_r_max_mm::mutation::ChangeLiquidSRMaxMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLiquidSRMaxMm, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { liquid_s_r_max_mm: Some(payload.new_liquid_s_r_max_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
