//! 🏢 `change-floor-category` diff: patches `category` of the addressed floor area row.

use crate::change_floor_category::ChangeFloorCategory;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FloorDelta, En1991FloorPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeFloorCategory, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.floors.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Floor area {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.category == payload.new_category {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The floor area already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { floors: En1991FloorDelta::modification(&payload.id, En1991FloorPatch { category: Some(payload.new_category.clone()), ..Default::default() }), ..Default::default() })
}
