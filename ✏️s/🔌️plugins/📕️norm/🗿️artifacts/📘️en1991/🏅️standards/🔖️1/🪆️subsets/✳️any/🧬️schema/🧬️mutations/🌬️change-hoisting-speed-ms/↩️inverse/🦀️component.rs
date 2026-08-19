//! ↩️ `change-hoisting-speed-ms` — undo restores BASE's hoisting speed.

use super::mutation::ChangeHoistingSpeedMS;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeHoistingSpeedMS, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeHoistingSpeedMS(ChangeHoistingSpeedMS { new_hoisting_speed_m_s: base.hoisting_speed_m_s.clone() })]
}
//#endregion 🔖️Inverse
