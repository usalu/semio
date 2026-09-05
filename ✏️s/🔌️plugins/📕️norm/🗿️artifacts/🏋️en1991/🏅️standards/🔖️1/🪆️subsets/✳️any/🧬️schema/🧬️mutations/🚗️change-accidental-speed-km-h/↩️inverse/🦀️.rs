//! ↩️ `change-accidental-speed-km-h` — undo restores BASE's accidental impact speed.

use super::ChangeAccidentalSpeedKmH;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAccidentalSpeedKmH, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAccidentalSpeedKmH(ChangeAccidentalSpeedKmH { new_accidental_speed_km_h: base.accidental_speed_km_h.clone() })]
}
//#endregion 🔖️Inverse
