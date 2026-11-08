//! ↩️ `change-critical-region-restr-bar-count` inverse: restores the previous `restrained_bar_count`.

use crate::change_critical_region_restr_bar_count::ChangeCriticalRegionRestrainedBarCount;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionRestrainedBarCount, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_restr_bar_count::ChangeCriticalRegionRestrainedBarCount> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionRestrainedBarCount, base: &En1998Snapshot) -> Option<ChangeCriticalRegionRestrainedBarCount> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionRestrainedBarCount { id: payload.id.clone(), new_restrained_bar_count: r0.restrained_bar_count })
}
