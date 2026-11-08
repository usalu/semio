//! 📍️ `change-critical-region-concrete-fcd` diff: patches `concrete_design_strength_mpa` of the addressed critical_region row.

use crate::change_critical_region_concrete_fcd::ChangeCriticalRegionConcreteDesignStrengthMpa;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998CriticalRegionDelta, En1998CriticalRegionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeCriticalRegionConcreteDesignStrengthMpa, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_concrete_design_strength_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.critical_regions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("critical_region {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.concrete_design_strength_mpa == payload.new_concrete_design_strength_mpa {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The critical_region already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { critical_regions: En1998CriticalRegionDelta::modification(&payload.id, En1998CriticalRegionPatch { concrete_design_strength_mpa: Some(payload.new_concrete_design_strength_mpa), ..Default::default() }), ..Default::default() })
}
