//! 🔺️ `insert-cold-formed-sheet` diff.

use crate::insert_cold_formed_sheet::InsertColdFormedSheet;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ColdFormedRows};

pub fn diff(payload: &InsertColdFormedSheet, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.cold_formed.iter().any(|existing| existing.id == payload.cold_formed_sheet.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Cold-formed sheet id {} already exists.", payload.cold_formed_sheet.id), [payload.cold_formed_sheet.id.clone()]);
    }
    if (payload.index as usize) > base.cold_formed.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.cold_formed.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { cold_formed: Some(En1999ColdFormedRows::insertion(payload.index as usize, payload.cold_formed_sheet.clone())), ..Default::default() })
}
