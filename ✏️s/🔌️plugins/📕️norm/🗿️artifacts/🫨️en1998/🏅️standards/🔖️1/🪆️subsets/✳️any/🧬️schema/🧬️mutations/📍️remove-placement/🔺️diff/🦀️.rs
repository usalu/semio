//! 📍️ `remove-placement` diff: removes the addressed placement row.

use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998PlacementDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemovePlacement, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.placements.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("placement {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { placements: En1998PlacementDelta::removal(&base.placements, index), ..Default::default() })
}
