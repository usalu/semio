//! ➕️ `insert-storey` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_storey::InsertStorey;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993StoreyDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertStorey, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.storeys.iter().any(|existing| existing.id == payload.storey.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Storey id {} already exists.", payload.storey.id), [payload.storey.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.storeys.len());
    protocol::MutationOutcome::new(En1993Diff { storeys: En1993StoreyDelta::insertion(index, payload.storey.clone()), ..Default::default() })
}
