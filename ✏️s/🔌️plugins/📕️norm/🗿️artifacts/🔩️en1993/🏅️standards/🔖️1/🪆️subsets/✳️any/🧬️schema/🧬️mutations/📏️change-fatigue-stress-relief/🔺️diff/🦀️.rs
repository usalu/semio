//! 🔺️ `change-fatigue-stress-relief` diff: patches `relief` of the addressed fatigue_stress row.

use crate::change_fatigue_stress_relief::ChangeFatigueStressRelief;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueStressDelta, En1993FatigueStressPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFatigueStressRelief, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.fatigue_stresses.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue_stress", Vec::<String>::new());
    };
    if row.relief == payload.new_relief {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue_stress already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fatigue_stresses: En1993FatigueStressDelta::modification(payload.id.clone(), En1993FatigueStressPatch { relief: Some(payload.new_relief), ..Default::default() }), ..Default::default() })
}
