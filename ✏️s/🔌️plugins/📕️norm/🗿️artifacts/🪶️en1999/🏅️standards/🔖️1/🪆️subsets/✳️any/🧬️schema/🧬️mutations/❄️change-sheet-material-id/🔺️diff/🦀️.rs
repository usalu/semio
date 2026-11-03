//! 🔺️ `change-sheet-material-id` diff: patches `material_id` of the addressed cold-formed sheet row.

use crate::change_sheet_material_id::ChangeSheetMaterialId;
use semio_s_artifact_norm_en1999_diff::{En1999ColdFormedPatch, En1999ColdFormedRows, En1999Diff};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeSheetMaterialId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "cold-formed", Vec::<String>::new());
    };
    if r0.material_id == payload.new_material_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The cold-formed sheet already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { cold_formed: Some(En1999ColdFormedRows::modification(&payload.cold_formed_id, En1999ColdFormedPatch { material_id: Some(payload.new_material_id.clone()), ..Default::default() })), ..Default::default() })
}
