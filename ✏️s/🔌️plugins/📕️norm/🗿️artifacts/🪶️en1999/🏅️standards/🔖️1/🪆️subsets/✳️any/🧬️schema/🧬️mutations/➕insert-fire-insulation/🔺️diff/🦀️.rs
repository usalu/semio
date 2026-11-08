//! 🔺️ `insert-fire-insulation` diff.

use crate::insert_fire_insulation::InsertFireInsulation;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FireInsulationsRows};

pub fn diff(payload: &InsertFireInsulation, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.fire_insulations.iter().any(|existing| existing.id == payload.fire_insulation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fire insulation id {} already exists.", payload.fire_insulation.id), [payload.fire_insulation.id.clone()]);
    }
    if (payload.index as usize) > base.fire_insulations.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.fire_insulations.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { fire_insulations: Some(En1999FireInsulationsRows::insertion(payload.index as usize, payload.fire_insulation.clone())), ..Default::default() })
}
