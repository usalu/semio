//! 🔺️ `change-zone-occupants` sparse diff.

use crate::change_zone_occupants::ChangeZoneOccupants;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ZonesRows, Din18599ZonesPatch};

pub fn diff(payload: &ChangeZoneOccupants, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if payload.new_occupants > 10000 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "occupants must lie from 0 to 10000.", Vec::<String>::new());
    }
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone \"{}\" does not exist.", payload.zone_id), [payload.zone_id.clone()]);
    };
    if row.occupants == payload.new_occupants {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Zone occupants already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        zones: Some(Din18599ZonesRows::modification(&payload.zone_id, Din18599ZonesPatch { occupants: Some(payload.new_occupants), ..Default::default() })),
        ..Default::default()
    })
}
