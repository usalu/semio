//! 🔺️ `change-fire-insulation-specific-heat` diff: patches `specific_heat` of the addressed connection row.

use crate::change_fire_insulation_specific_heat::ChangeFireInsulationSpecificHeat;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FireInsulationsPatch, En1999FireInsulationsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeFireInsulationSpecificHeat, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_specific_heat.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.fire_insulations.iter().find(|row| row.id == payload.fire_insulation_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fire-insulation", Vec::<String>::new());
    };
    if r0.specific_heat == payload.new_specific_heat {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The connection already has this hole stagger.");
    }
    protocol::MutationOutcome::new(En1999Diff { fire_insulations: Some(En1999FireInsulationsRows::modification(&payload.fire_insulation_id, En1999FireInsulationsPatch { specific_heat: Some(payload.new_specific_heat), ..Default::default() })), ..Default::default() })
}
