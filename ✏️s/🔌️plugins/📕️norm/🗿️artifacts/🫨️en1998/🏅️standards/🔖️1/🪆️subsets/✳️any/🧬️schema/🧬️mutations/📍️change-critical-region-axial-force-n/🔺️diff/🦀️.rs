//! 📍️ `change-critical-region-axial-force-n` diff: patches `axial_force_n` of the addressed critical_region row.

use crate::change_critical_region_axial_force_n::ChangeCriticalRegionAxialForceN;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998CriticalRegionDelta, En1998CriticalRegionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeCriticalRegionAxialForceN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_axial_force_n.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.critical_regions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("critical_region {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.axial_force_n == payload.new_axial_force_n {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The critical_region already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { critical_regions: En1998CriticalRegionDelta::modification(&payload.id, En1998CriticalRegionPatch { axial_force_n: Some(payload.new_axial_force_n), ..Default::default() }), ..Default::default() })
}
