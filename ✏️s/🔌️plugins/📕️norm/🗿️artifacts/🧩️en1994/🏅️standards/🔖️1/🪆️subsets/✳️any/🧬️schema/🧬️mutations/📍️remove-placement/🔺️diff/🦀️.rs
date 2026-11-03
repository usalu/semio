//! 📍️ `remove-placement` diff on the positional `placements` lane.

use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994PlacementsRows};
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn diff(payload: &RemovePlacement, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.placements.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("placement {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if index >= base.placements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "placements index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1994Diff { placements: Some(En1994PlacementsRows::removal(&base.placements, index)), ..Default::default() })
}
