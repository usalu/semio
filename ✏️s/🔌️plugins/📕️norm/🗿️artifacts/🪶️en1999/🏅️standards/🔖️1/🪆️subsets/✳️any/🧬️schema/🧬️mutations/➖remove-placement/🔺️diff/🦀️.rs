//! 🔺️ `remove-placement` diff.

use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999PlacementsRows};

pub fn diff(payload: &RemovePlacement, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.placements.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown placement id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { placements: Some(En1999PlacementsRows::removal(&base.placements, index)), ..Default::default() })
}
