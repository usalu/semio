//! ➖️ `remove-placement` diff — removes the row at the index.

use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PlacementDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemovePlacement, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.placements.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("placement index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { placements: En1993PlacementDelta::removal(&base.placements, payload.index), ..Default::default() })
}
