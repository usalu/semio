//! 🔺️ `change-wall-soil-gamma-kn-m3` sparse diff construction — writes only `En1998Diff.wall_soil_gamma_kn_m3` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_wall_soil_gamma_kn_m3::mutation::ChangeWallSoilGammaKnM3;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWallSoilGammaKnM3, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { wall_soil_gamma_kn_m3: Some(payload.new_wall_soil_gamma_kn_m3.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
