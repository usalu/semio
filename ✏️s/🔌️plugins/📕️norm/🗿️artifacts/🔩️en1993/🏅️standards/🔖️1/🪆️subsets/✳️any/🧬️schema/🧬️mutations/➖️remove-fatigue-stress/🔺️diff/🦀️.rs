//! ➖️ `remove-fatigue-stress` diff — removes the row at the index.

use crate::remove_fatigue_stress::RemoveFatigueStress;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueStressDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveFatigueStress, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.fatigue_stresses.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("fatigue-stress index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { fatigue_stresses: En1993FatigueStressDelta::removal(&base.fatigue_stresses, payload.index), ..Default::default() })
}
