//! 🔺️ `remove-fire-insulation` diff.

use crate::remove_fire_insulation::RemoveFireInsulation;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FireInsulationsRows};

pub fn diff(payload: &RemoveFireInsulation, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.fire_insulations.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown fire insulation id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { fire_insulations: Some(En1999FireInsulationsRows::removal(&base.fire_insulations, index)), ..Default::default() })
}
