//! 📍️ `change-critical-region-long-steel-area` diff: patches `longitudinal_steel_area_m2` of the addressed critical_region row.

use crate::change_critical_region_long_steel_area::ChangeCriticalRegionLongitudinalSteelAreaM2;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998CriticalRegionDelta, En1998CriticalRegionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeCriticalRegionLongitudinalSteelAreaM2, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_longitudinal_steel_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.critical_regions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("critical_region {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.longitudinal_steel_area_m2 == payload.new_longitudinal_steel_area_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The critical_region already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { critical_regions: En1998CriticalRegionDelta::modification(&payload.id, En1998CriticalRegionPatch { longitudinal_steel_area_m2: Some(payload.new_longitudinal_steel_area_m2), ..Default::default() }), ..Default::default() })
}
