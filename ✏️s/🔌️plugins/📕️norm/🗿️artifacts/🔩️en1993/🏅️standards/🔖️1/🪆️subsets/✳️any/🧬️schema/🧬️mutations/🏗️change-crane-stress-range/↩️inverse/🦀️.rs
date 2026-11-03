//! ↩️ `change-crane-stress-range` inverse: restores the previous `stress_range`.

use crate::change_crane_stress_range::ChangeCraneStressRange;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneStressRange, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_stress_range::ChangeCraneStressRange> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneStressRange { id: payload.id.clone(), new_stress_range: row.stress_range })],
        None => Vec::new(),
    })
}
