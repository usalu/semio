//! 🔺️ `change-fatigue-stress-max-case-id` diff: patches `max_case_id` of the addressed fatigue_stress block row.

use crate::change_fatigue_stress_max_case_id::ChangeFatigueStressMaxCaseId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueStressDelta, En1993FatigueStressPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFatigueStressMaxCaseId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.fatigue_stresses.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-stress", Vec::<String>::new());
    };
    if row.max_case_id == payload.new_max_case_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue_stress block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fatigue_stresses: En1993FatigueStressDelta::modification(payload.id.clone(), En1993FatigueStressPatch { max_case_id: Some(payload.new_max_case_id.clone()), ..Default::default() }), ..Default::default() })
}
