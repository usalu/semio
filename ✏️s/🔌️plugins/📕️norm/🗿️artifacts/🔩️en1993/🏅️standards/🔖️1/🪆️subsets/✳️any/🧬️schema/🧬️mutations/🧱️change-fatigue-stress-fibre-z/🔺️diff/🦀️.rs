//! 🔺️ `change-fatigue-stress-fibre-z` diff: patches `fibre_z` of the addressed fatigue_stress block row.

use crate::change_fatigue_stress_fibre_z::ChangeFatigueStressFibreZ;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueStressDelta, En1993FatigueStressPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFatigueStressFibreZ, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_fibre_z.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.fatigue_stresses.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-stress", Vec::<String>::new());
    };
    if row.fibre_z == payload.new_fibre_z {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue_stress block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fatigue_stresses: En1993FatigueStressDelta::modification(payload.id.clone(), En1993FatigueStressPatch { fibre_z: Some(payload.new_fibre_z), ..Default::default() }), ..Default::default() })
}
