//! 🗺️ `change-floor-area` diff: patches `area` of the addressed floor row.

use crate::change_floor_area::ChangeFloorArea;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FloorDelta, En1991FloorPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeFloorArea, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_area.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.floors.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("floor {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.area == payload.new_area {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The floor already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { floors: En1991FloorDelta::modification(&payload.id, En1991FloorPatch { area: Some(payload.new_area), ..Default::default() }), ..Default::default() })
}
