//! 🔺️ `change-hoisting-speed-ms` — sparse diff construction.

use super::mutation::ChangeHoistingSpeedMS;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeHoistingSpeedMS, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { hoisting_speed_m_s: Some(payload.new_hoisting_speed_m_s.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
