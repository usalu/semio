//! 📍️ `remove-placement` diff: removes the addressed placement row.

use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996PlacementDelta};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &RemovePlacement, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(index) = base.placements.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("placement {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1996Diff { placements: En1996PlacementDelta::removal(&base.placements, index), ..Default::default() })
}
