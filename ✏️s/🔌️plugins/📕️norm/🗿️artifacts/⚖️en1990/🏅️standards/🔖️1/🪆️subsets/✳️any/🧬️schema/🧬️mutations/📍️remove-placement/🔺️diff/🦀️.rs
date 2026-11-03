//! 📍 `remove-placement` diff: removes the addressed placement row.

use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990PlacementDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &RemovePlacement, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let index = payload.index;
    if index >= base.placements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "placements index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1990Diff { placements: En1990PlacementDelta::removal(&base.placements, index), ..Default::default() })
}
