//! ↩️ `change-fatigue-stress-range` inverse: restores the previous `stress_range`.

use crate::change_fatigue_stress_range::ChangeFatigueStressRange;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueStressRange, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_stress_range::ChangeFatigueStressRange> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFatigueStressRange, base: &En1990Snapshot) -> Option<ChangeFatigueStressRange> {
    let r0 = base.fatigue_spectra.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFatigueStressRange { id: payload.id.clone(), new_stress_range: r0.stress_range })
}
