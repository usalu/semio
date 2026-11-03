//! 📍️ `remove-placement` diff: removes the addressed placement row.

use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991PlacementDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemovePlacement, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.placements.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("placement {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { placements: En1991PlacementDelta::removal(&base.placements, index), ..Default::default() })
}
