//! 📍️ `remove-placement` diff: removes the addressed placement row.

use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PlacementsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &RemovePlacement, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let index = payload.index;
    if index >= base.placements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "placements index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { placements: Some(En1997PlacementsRows::removal(&base.placements, index)), ..Default::default() })
}
