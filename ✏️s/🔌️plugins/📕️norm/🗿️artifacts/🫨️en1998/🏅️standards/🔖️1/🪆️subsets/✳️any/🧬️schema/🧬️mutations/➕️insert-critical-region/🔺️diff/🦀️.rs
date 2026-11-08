//! ➕️ `insert-critical-region` diff: inserts the critical_region row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_critical_region::InsertCriticalRegion;
use semio_s_artifact_norm_en1998_diff::{En1998CriticalRegionDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertCriticalRegion, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.critical_regions.iter().any(|existing| existing.id == payload.critical_region.id) {
        let key = payload.critical_region.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("critical_region {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.critical_regions.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { critical_regions: En1998CriticalRegionDelta::insertion(index, payload.critical_region.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the critical_region list; inserted at {index}.", payload.index.unwrap_or(index)))
}
