//! 🏠️ `change-roof-surface-extent` diff: patches `extent` of the addressed roof surface row.

use crate::change_roof_surface_extent::ChangeRoofSurfaceExtent;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofSurfaceDelta, En1991RoofSurfacePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRoofSurfaceExtent, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_extent.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.roof_surfaces.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("roof surface {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.extent == payload.new_extent {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The roof surface already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { roof_surfaces: En1991RoofSurfaceDelta::modification(&payload.id, En1991RoofSurfacePatch { extent: Some(payload.new_extent), ..Default::default() }), ..Default::default() })
}
