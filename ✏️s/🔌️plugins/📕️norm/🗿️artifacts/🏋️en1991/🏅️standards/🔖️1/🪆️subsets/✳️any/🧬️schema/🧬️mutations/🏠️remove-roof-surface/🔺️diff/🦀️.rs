//! 🏠️ `remove-roof-surface` diff: removes the addressed roof surface row.

use crate::remove_roof_surface::RemoveRoofSurface;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofSurfaceDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveRoofSurface, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.roof_surfaces.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("roof surface {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { roof_surfaces: En1991RoofSurfaceDelta::removal(&base.roof_surfaces, index), ..Default::default() })
}
