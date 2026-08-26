//! ↩️ `change-wall-soil-gamma-kn-m3` inverse — restores the pre-change `wall_soil_gamma_kn_m3` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_wall_soil_gamma_kn_m3::mutation::ChangeWallSoilGammaKnM3;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWallSoilGammaKnM3, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeWallSoilGammaKnM3(ChangeWallSoilGammaKnM3 { new_wall_soil_gamma_kn_m3: base.wall_soil_gamma_kn_m3.clone() })]
}
//#endregion 🔖️Inverse
