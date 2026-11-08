//! ➖️ `remove-critical-region` diff: removes the addressed critical_region row.

use crate::remove_critical_region::RemoveCriticalRegion;
use semio_s_artifact_norm_en1998_diff::{En1998CriticalRegionDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveCriticalRegion, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.critical_regions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("critical_region {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { critical_regions: En1998CriticalRegionDelta::removal(&base.critical_regions, index), ..Default::default() })
}
