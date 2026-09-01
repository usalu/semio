//! 🔺️ `change-snow-altitude-m` — sparse diff construction.

use super::ChangeSnowAltitudeM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSnowAltitudeM, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_snow_altitude_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Snow altitude m must be a finite number.", Vec::<String>::new());
    }
    if base.snow_altitude_m == payload.new_snow_altitude_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Snow altitude m already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { snow_altitude_m: Some(payload.new_snow_altitude_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
