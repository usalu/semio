//! 📍️ `remove-placement` diff: removes the addressed placement row.

use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992PlacementsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &RemovePlacement, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let index = payload.index;
    if index >= base.placements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "placements index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1992Diff { placements: Some(En1992PlacementsRows::removal(&base.placements, index)), ..Default::default() })
}
