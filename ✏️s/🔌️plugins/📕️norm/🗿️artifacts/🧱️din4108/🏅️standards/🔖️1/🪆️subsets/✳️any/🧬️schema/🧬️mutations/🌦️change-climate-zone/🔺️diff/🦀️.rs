//! 🔺️ `change-climate-zone` diff.

use super::ChangeClimateZone;
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeClimateZone, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.climate_zone == payload.new_climate_zone {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "climate_zone already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { climate_zone: Some(payload.new_climate_zone), ..Default::default() })
}
