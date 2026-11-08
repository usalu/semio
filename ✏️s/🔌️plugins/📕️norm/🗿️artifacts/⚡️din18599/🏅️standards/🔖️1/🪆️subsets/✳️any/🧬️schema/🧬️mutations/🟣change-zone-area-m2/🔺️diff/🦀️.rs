//! 🔺️ `change-zone-area-m2` sparse diff.

use crate::change_zone_area_m2::ChangeZoneAreaM2;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ZonesRows, Din18599ZonesPatch};

pub fn diff(payload: &ChangeZoneAreaM2, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_area_m2.is_finite() || payload.new_area_m2 <= 0.0 || payload.new_area_m2 > 1000000.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "area-m2 must lie above 0 to 1e+06.", Vec::<String>::new());
    }
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone \"{}\" does not exist.", payload.zone_id), [payload.zone_id.clone()]);
    };
    if row.area_m2 == payload.new_area_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Zone area m2 already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        zones: Some(Din18599ZonesRows::modification(&payload.zone_id, Din18599ZonesPatch { area_m2: Some(payload.new_area_m2), ..Default::default() })),
        ..Default::default()
    })
}
