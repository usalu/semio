//! ↩️ `change-zone-occupants` inverse.

use crate::change_zone_occupants::ChangeZoneOccupants;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneOccupants, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_occupants::ChangeZoneOccupants> {
    Ok(base.zones.iter().find(|row| row.id == payload.zone_id).map(|row| vec![Op::from(ChangeZoneOccupants { zone_id: payload.zone_id.clone(), new_occupants: row.occupants })]).unwrap_or_default())
}
