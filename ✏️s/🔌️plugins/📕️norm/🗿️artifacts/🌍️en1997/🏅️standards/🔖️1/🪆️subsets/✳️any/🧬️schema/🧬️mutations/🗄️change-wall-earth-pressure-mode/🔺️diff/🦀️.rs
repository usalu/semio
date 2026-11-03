//! 🔺️ `change-wall-earth-pressure-mode` diff: patches `earth_pressure_mode` of the addressed retaining wall row.

use crate::change_wall_earth_pressure_mode::ChangeWallEarthPressureMode;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997RetainingWallsPatch, En1997RetainingWallsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeWallEarthPressureMode, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(r0) = base.retaining_walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.earth_pressure_mode == payload.new_earth_pressure_mode {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The retaining wall already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { retaining_walls: Some(En1997RetainingWallsRows::modification(&payload.id, En1997RetainingWallsPatch { earth_pressure_mode: Some(payload.new_earth_pressure_mode), ..Default::default() })), ..Default::default() })
}
