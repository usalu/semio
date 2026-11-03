//! 🏠️ `change-roof-surface-shape` diff: patches `shape` of the addressed roof surface row.

use crate::change_roof_surface_shape::ChangeRoofSurfaceShape;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofSurfaceDelta, En1991RoofSurfacePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRoofSurfaceShape, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.roof_surfaces.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("roof surface {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.shape == payload.new_shape {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The roof surface already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { roof_surfaces: En1991RoofSurfaceDelta::modification(&payload.id, En1991RoofSurfacePatch { shape: Some(payload.new_shape.clone()), ..Default::default() }), ..Default::default() })
}
