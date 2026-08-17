//! 🔺️ `change-wind-zone` — sparse diff construction.

use super::mutation::ChangeWindZone;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeWindZone, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.wind_zone == payload.new_wind_zone {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Wind zone already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { wind_zone: Some(payload.new_wind_zone.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
