//! 🔺️ `change-gamma-kn-m3` sparse diff construction — writes only `En1997Diff.gamma_kn_m3` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_gamma_kn_m3::mutation::ChangeGammaKnM3;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeGammaKnM3, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { gamma_kn_m3: Some(payload.new_gamma_kn_m3.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
