//! 🔺️ `change-climate-zone` diff.

use crate::change_climate_zone::ChangeClimateZone;
use semio_s_artifact_norm_din4108_diff::Din4108Diff;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn diff(payload: &ChangeClimateZone, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.climate_zone == payload.new_climate_zone {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "climate_zone already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { climate_zone: Some(payload.new_climate_zone), ..Default::default() })
}
