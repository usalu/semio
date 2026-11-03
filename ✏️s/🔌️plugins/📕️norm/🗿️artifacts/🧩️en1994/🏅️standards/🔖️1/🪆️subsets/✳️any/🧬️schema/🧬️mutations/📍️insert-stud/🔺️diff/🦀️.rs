//! 📍️ `insert-stud` diff on the positional `studs` lane.

use crate::insert_stud::InsertStud;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994StudsInserted, En1994StudsRows};
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn diff(payload: &InsertStud, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if base.studs.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Shear shear stud {key} already exists."), [key]);
    }
    if payload.index > base.studs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.studs.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1994Diff { studs: Some(En1994StudsRows::insertion(payload.index, payload.item.clone())), ..Default::default() })
}
