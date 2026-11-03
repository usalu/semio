//! ➖️ `remove-storey` diff — removes the row at the index.

use crate::remove_storey::RemoveStorey;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993StoreyDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveStorey, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.storeys.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("storey index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { storeys: En1993StoreyDelta::removal(&base.storeys, payload.index), ..Default::default() })
}
