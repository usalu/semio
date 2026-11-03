//! 🔺️ `change-fatigue-stress-min-case-id` diff: patches `min_case_id` of the addressed fatigue_stress block row.

use crate::change_fatigue_stress_min_case_id::ChangeFatigueStressMinCaseId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueStressDelta, En1993FatigueStressPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFatigueStressMinCaseId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.fatigue_stresses.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-stress", Vec::<String>::new());
    };
    if row.min_case_id == payload.new_min_case_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue_stress block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fatigue_stresses: En1993FatigueStressDelta::modification(payload.id.clone(), En1993FatigueStressPatch { min_case_id: Some(payload.new_min_case_id.clone()), ..Default::default() }), ..Default::default() })
}
