//! Diff for `change-snow-zone`.
use super::ChangeSnowZone;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeSnowZone, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.snow_zone == payload.new_snow_zone {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { snow_zone: Some(payload.new_snow_zone.clone()), ..Default::default() })
}
