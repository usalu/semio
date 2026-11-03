//! 🔺️ `change-fire-prot-specific-heat` diff: patches `protection_specific_heat` of the addressed fire exposure row.

use crate::change_fire_prot_specific_heat::ChangeFireProtSpecificHeat;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FireExposureDelta, En1993FireExposurePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFireProtSpecificHeat, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_protection_specific_heat.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.fire_exposures.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fire-exposure", Vec::<String>::new());
    };
    if row.protection_specific_heat == payload.new_protection_specific_heat {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fire exposure already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fire_exposures: En1993FireExposureDelta::modification(payload.id.clone(), En1993FireExposurePatch { protection_specific_heat: Some(payload.new_protection_specific_heat), ..Default::default() }), ..Default::default() })
}
