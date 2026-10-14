use crate::insert_footing::InsertFooting;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997FootingsRows};

pub fn diff(payload: &InsertFooting, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.footings.iter().any(|existing| existing.id == payload.footing.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Footing id {} already exists.", payload.footing.id), [payload.footing.id.clone()]);
    }
    if payload.index > base.footings.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.footings.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingsRows::insertion(payload.index, payload.footing.clone())), ..Default::default() })
}
