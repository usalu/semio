//! ↩️ `change-snow-altitude-m` — undo restores BASE's snow altitude.

use super::ChangeSnowAltitudeM;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSnowAltitudeM, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSnowAltitudeM(ChangeSnowAltitudeM { new_snow_altitude_m: base.snow_altitude_m.clone() })]
}
//#endregion 🔖️Inverse
