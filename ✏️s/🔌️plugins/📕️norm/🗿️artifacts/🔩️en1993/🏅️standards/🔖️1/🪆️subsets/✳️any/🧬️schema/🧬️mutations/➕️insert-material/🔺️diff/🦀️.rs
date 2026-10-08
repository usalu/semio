//! ➕️ `insert-material` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertMaterial;
use crate::diff::{En1993Diff, En1993MaterialDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertMaterial, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.materials.iter().any(|existing| existing.id == payload.material.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Material id {} already exists.", payload.material.id), [payload.material.id.clone()]);
    }
    let index = payload.index.min(base.materials.len());
    protocol::MutationOutcome::new(En1993Diff { materials: En1993MaterialDelta::insertion(&base.materials, index, payload.material.clone()), ..Default::default() })
}
