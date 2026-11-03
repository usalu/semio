//! ➕️ `insert-fatigue-stress` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_fatigue_stress::InsertFatigueStress;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueStressDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertFatigueStress, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.fatigue_stresses.iter().any(|existing| existing.id == payload.fatigue_stress.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Stress range id {} already exists.", payload.fatigue_stress.id), [payload.fatigue_stress.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.fatigue_stresses.len());
    protocol::MutationOutcome::new(En1993Diff { fatigue_stresses: En1993FatigueStressDelta::insertion(index, payload.fatigue_stress.clone()), ..Default::default() })
}
