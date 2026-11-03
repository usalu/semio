//! 🏠️ `insert-roof-surface` diff: inserts the roof surface row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_roof_surface::InsertRoofSurface;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofSurfaceDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertRoofSurface, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.roof_surfaces.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("roof surface {key} already exists."), [key]);
    }
    if payload.index > base.roof_surfaces.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.roof_surfaces.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { roof_surfaces: En1991RoofSurfaceDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
