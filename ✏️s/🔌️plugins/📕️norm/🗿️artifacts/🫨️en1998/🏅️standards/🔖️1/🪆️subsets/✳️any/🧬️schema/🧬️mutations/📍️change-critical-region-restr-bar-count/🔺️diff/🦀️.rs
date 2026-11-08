//! 📍️ `change-critical-region-restr-bar-count` diff: patches `restrained_bar_count` of the addressed critical_region row.

use crate::change_critical_region_restr_bar_count::ChangeCriticalRegionRestrainedBarCount;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998CriticalRegionDelta, En1998CriticalRegionPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeCriticalRegionRestrainedBarCount, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_restrained_bar_count.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.critical_regions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("critical_region {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.restrained_bar_count == payload.new_restrained_bar_count {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The critical_region already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { critical_regions: En1998CriticalRegionDelta::modification(&payload.id, En1998CriticalRegionPatch { restrained_bar_count: Some(payload.new_restrained_bar_count), ..Default::default() }), ..Default::default() })
}
