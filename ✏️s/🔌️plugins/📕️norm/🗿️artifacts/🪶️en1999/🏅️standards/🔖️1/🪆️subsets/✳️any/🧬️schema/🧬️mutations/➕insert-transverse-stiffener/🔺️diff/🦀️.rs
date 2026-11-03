//! 🔺️ `insert-transverse-stiffener` diff.

use crate::insert_transverse_stiffener::InsertTransverseStiffener;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999TransverseStiffenersRows};

pub fn diff(payload: &InsertTransverseStiffener, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.transverse_stiffeners.iter().any(|existing| existing.id == payload.transverse_stiffener.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Transverse stiffener id {} already exists.", payload.transverse_stiffener.id), [payload.transverse_stiffener.id.clone()]);
    }
    if (payload.index as usize) > base.transverse_stiffeners.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.transverse_stiffeners.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { transverse_stiffeners: Some(En1999TransverseStiffenersRows::insertion(payload.index as usize, payload.transverse_stiffener.clone())), ..Default::default() })
}
