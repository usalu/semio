//! 🔺️ `change-accidental-speed-km-h` — sparse diff construction.

use super::mutation::ChangeAccidentalSpeedKmH;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAccidentalSpeedKmH, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { accidental_speed_km_h: Some(payload.new_accidental_speed_km_h.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
