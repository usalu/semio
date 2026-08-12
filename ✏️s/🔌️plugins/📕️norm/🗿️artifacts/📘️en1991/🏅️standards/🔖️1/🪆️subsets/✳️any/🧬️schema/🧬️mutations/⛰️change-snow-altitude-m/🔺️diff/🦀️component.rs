//! 🔺️ `change-snow-altitude-m` — sparse diff construction.

use super::mutation::ChangeSnowAltitudeM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSnowAltitudeM, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { snow_altitude_m: Some(payload.new_snow_altitude_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
