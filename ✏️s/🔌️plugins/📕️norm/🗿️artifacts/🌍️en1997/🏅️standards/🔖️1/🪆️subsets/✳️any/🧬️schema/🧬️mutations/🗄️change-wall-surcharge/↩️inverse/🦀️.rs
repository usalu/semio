//! ↩️ `change-wall-surcharge` inverse: restores the previous `surcharge`.

use crate::change_wall_surcharge::ChangeWallSurcharge;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallSurcharge, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_surcharge::ChangeWallSurcharge> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallSurcharge { id: payload.id.clone(), new_surcharge: row.surcharge })],
        None => Vec::new(),
    })
}
