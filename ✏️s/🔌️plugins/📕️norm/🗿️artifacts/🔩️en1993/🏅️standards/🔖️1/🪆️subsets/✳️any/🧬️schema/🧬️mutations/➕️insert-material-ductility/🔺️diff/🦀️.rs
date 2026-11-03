//! ➕️ `insert-material-ductility` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_material_ductility::InsertMaterialDuctility;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MaterialDuctilityDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertMaterialDuctility, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.material_ductility.iter().any(|existing| existing.id == payload.material_ductility.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Material ductility id {} already exists.", payload.material_ductility.id), [payload.material_ductility.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.material_ductility.len());
    protocol::MutationOutcome::new(En1993Diff { material_ductility: En1993MaterialDuctilityDelta::insertion(index, payload.material_ductility.clone()), ..Default::default() })
}
