//! ↩️ `change-silo-length` inverse: restores the previous `length`.

use crate::change_silo_length::ChangeSiloLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_length::ChangeSiloLength> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloLength { id: payload.id.clone(), new_length: row.length })],
        None => Vec::new(),
    })
}
