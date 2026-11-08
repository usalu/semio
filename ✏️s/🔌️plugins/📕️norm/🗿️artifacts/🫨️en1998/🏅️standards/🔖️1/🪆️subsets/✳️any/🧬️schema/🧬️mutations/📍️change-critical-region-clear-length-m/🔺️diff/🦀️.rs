//! 📍️ `change-critical-region-clear-length-m` diff: patches `clear_length_m` of the addressed critical_region row.

use crate::change_critical_region_clear_length_m::ChangeCriticalRegionClearLengthM;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998CriticalRegionDelta, En1998CriticalRegionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeCriticalRegionClearLengthM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_clear_length_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.critical_regions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("critical_region {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.clear_length_m == payload.new_clear_length_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The critical_region already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { critical_regions: En1998CriticalRegionDelta::modification(&payload.id, En1998CriticalRegionPatch { clear_length_m: Some(payload.new_clear_length_m), ..Default::default() }), ..Default::default() })
}
