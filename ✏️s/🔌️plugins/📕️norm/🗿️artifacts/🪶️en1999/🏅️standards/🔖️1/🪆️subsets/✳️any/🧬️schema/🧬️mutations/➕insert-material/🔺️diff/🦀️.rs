//! 🔺️ `insert-material` diff.

use crate::insert_material::InsertMaterial;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MaterialsRows};

pub fn diff(payload: &InsertMaterial, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.materials.iter().any(|existing| existing.id == payload.material.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Material id {} already exists.", payload.material.id), [payload.material.id.clone()]);
    }
    if (payload.index as usize) > base.materials.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.materials.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { materials: Some(En1999MaterialsRows::insertion(payload.index as usize, payload.material.clone())), ..Default::default() })
}
