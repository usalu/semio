//! 🔺️ `change-snow-zone` — sparse diff construction.

use super::ChangeSnowZone;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSnowZone, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.snow_zone == payload.new_snow_zone {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Snow zone already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { snow_zone: Some(payload.new_snow_zone.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
