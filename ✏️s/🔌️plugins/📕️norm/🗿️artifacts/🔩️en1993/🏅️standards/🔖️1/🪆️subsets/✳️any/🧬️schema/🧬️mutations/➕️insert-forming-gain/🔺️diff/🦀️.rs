//! ➕️ `insert-forming-gain` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_forming_gain::InsertFormingGain;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FormingGainDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertFormingGain, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.forming_gains.iter().any(|existing| existing.id == payload.forming_gain.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Forming gain id {} already exists.", payload.forming_gain.id), [payload.forming_gain.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.forming_gains.len());
    protocol::MutationOutcome::new(En1993Diff { forming_gains: En1993FormingGainDelta::insertion(index, payload.forming_gain.clone()), ..Default::default() })
}
