//! 🔺️ `change-zone-volume-m3` sparse diff.

use crate::change_zone_volume_m3::ChangeZoneVolumeM3;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ZonesRows, Din18599ZonesPatch};

pub fn diff(payload: &ChangeZoneVolumeM3, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_volume_m3.is_finite() || payload.new_volume_m3 <= 0.0 || payload.new_volume_m3 > 10000000.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "volume-m3 must lie above 0 to 1e+07.", Vec::<String>::new());
    }
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone \"{}\" does not exist.", payload.zone_id), [payload.zone_id.clone()]);
    };
    if row.volume_m3 == payload.new_volume_m3 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Zone volume m3 already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        zones: Some(Din18599ZonesRows::modification(&payload.zone_id, Din18599ZonesPatch { volume_m3: Some(payload.new_volume_m3), ..Default::default() })),
        ..Default::default()
    })
}
