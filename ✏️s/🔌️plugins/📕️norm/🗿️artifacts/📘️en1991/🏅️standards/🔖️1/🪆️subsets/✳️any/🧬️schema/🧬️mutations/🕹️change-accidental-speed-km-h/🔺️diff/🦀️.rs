//! 🔺️ `change-accidental-speed-km-h` — sparse diff construction.

use super::ChangeAccidentalSpeedKmH;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAccidentalSpeedKmH, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_accidental_speed_km_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Accidental speed km h must be a finite number.", Vec::<String>::new());
    }
    if base.accidental_speed_km_h == payload.new_accidental_speed_km_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Accidental speed km h already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { accidental_speed_km_h: Some(payload.new_accidental_speed_km_h.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
