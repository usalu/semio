use crate::insert_slope::InsertSlope;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997SlopesRows};

pub fn diff(payload: &InsertSlope, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.slopes.iter().any(|existing| existing.id == payload.slope.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Slope id {} already exists.", payload.slope.id), [payload.slope.id.clone()]);
    }
    if payload.index > base.slopes.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.slopes.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { slopes: Some(En1997SlopesRows::insertion(payload.index, payload.slope.clone())), ..Default::default() })
}
