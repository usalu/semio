//! ✏️ `change-retaining-wall-water-height-m` diff — patches the one field of the addressed row; a missing target is a `mutation.target-missing`.

use crate::change_retaining_wall_water_height_m::ChangeRetainingWallWaterHeightM;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998RetainingWallDelta, En1998RetainingWallPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeRetainingWallWaterHeightM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.retaining_walls.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "retaining wall", Vec::<String>::new());
    };
    let patch = En1998RetainingWallPatch { water_height_m: Some(payload.new_water_height_m), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { retaining_walls: En1998RetainingWallDelta::modification(&row.id, patch), ..Default::default() })
}
