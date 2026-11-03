//! 🏠️ `change-roof-surface-face` diff: patches `face` of the addressed roof surface row.

use crate::change_roof_surface_face::ChangeRoofSurfaceFace;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofSurfaceDelta, En1991RoofSurfacePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRoofSurfaceFace, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.roof_surfaces.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("roof surface {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.face == payload.new_face {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The roof surface already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { roof_surfaces: En1991RoofSurfaceDelta::modification(&payload.id, En1991RoofSurfacePatch { face: Some(payload.new_face.clone()), ..Default::default() }), ..Default::default() })
}
