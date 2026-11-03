//! ↩️ `change-crane-load-factor` inverse: restores the previous `load_factor`.

use crate::change_crane_load_factor::ChangeCraneLoadFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneLoadFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_load_factor::ChangeCraneLoadFactor> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneLoadFactor { id: payload.id.clone(), new_load_factor: row.load_factor })],
        None => Vec::new(),
    })
}
