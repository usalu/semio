//! ➕️ `insert-material` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_material::InsertMaterial;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MaterialDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertMaterial, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.materials.iter().any(|existing| existing.id == payload.material.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Material id {} already exists.", payload.material.id), [payload.material.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.materials.len());
    protocol::MutationOutcome::new(En1993Diff { materials: En1993MaterialDelta::insertion(index, payload.material.clone()), ..Default::default() })
}
