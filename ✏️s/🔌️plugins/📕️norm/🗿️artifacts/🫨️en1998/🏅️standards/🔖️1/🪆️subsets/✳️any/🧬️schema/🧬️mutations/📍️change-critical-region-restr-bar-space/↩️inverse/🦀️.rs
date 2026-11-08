//! ↩️ `change-critical-region-restr-bar-space` inverse: restores the previous `restrained_bar_spacing_m`.

use crate::change_critical_region_restr_bar_space::ChangeCriticalRegionRestrainedBarSpacingM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionRestrainedBarSpacingM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_restr_bar_space::ChangeCriticalRegionRestrainedBarSpacingM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionRestrainedBarSpacingM, base: &En1998Snapshot) -> Option<ChangeCriticalRegionRestrainedBarSpacingM> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionRestrainedBarSpacingM { id: payload.id.clone(), new_restrained_bar_spacing_m: r0.restrained_bar_spacing_m })
}
