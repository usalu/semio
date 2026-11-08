//! 📍️ `change-critical-region-hoop-fywd` diff: patches `hoop_design_yield_mpa` of the addressed critical_region row.

use crate::change_critical_region_hoop_fywd::ChangeCriticalRegionHoopDesignYieldMpa;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998CriticalRegionDelta, En1998CriticalRegionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeCriticalRegionHoopDesignYieldMpa, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_hoop_design_yield_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.critical_regions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("critical_region {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.hoop_design_yield_mpa == payload.new_hoop_design_yield_mpa {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The critical_region already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { critical_regions: En1998CriticalRegionDelta::modification(&payload.id, En1998CriticalRegionPatch { hoop_design_yield_mpa: Some(payload.new_hoop_design_yield_mpa), ..Default::default() }), ..Default::default() })
}
