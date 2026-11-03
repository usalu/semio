//! ↩️ `change-crane-dispersion` inverse: restores the previous `dispersion`.

use crate::change_crane_dispersion::ChangeCraneDispersion;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneDispersion, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_dispersion::ChangeCraneDispersion> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneDispersion { id: payload.id.clone(), new_dispersion: row.dispersion })],
        None => Vec::new(),
    })
}
