//! 🔺️ `change-sheet-width` diff: patches `width` of the addressed cold-formed sheet row.

use crate::change_sheet_width::ChangeSheetWidth;
use semio_s_artifact_norm_en1999_diff::{En1999ColdFormedPatch, En1999ColdFormedRows, En1999Diff};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeSheetWidth, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_width.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "cold-formed", Vec::<String>::new());
    };
    if r0.width == payload.new_width {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The cold-formed sheet already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { cold_formed: Some(En1999ColdFormedRows::modification(&payload.cold_formed_id, En1999ColdFormedPatch { width: Some(payload.new_width), ..Default::default() })), ..Default::default() })
}
