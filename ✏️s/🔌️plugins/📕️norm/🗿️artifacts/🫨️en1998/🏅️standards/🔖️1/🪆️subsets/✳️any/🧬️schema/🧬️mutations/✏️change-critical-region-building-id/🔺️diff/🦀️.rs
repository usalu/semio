//! ✏️ `change-critical-region-building-id` diff: patches `building_id` of the addressed critical_region row.

use crate::change_critical_region_building_id::ChangeCriticalRegionBuildingId;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998CriticalRegionDelta, En1998CriticalRegionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeCriticalRegionBuildingId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.critical_regions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("critical_region {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.building_id == payload.new_building_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The critical_region already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { critical_regions: En1998CriticalRegionDelta::modification(&payload.id, En1998CriticalRegionPatch { building_id: Some(payload.new_building_id.clone()), ..Default::default() }), ..Default::default() })
}
