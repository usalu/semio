//! 🔺️ `change-sheet-thickness` diff: patches `thickness` of the addressed cold-formed sheet row.

use crate::change_sheet_thickness::ChangeSheetThickness;
use semio_s_artifact_norm_en1999_diff::{En1999ColdFormedPatch, En1999ColdFormedRows, En1999Diff};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeSheetThickness, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_thickness.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "cold-formed", Vec::<String>::new());
    };
    if r0.thickness == payload.new_thickness {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The cold-formed sheet already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { cold_formed: Some(En1999ColdFormedRows::modification(&payload.cold_formed_id, En1999ColdFormedPatch { thickness: Some(payload.new_thickness), ..Default::default() })), ..Default::default() })
}
