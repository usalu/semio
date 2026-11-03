//! ↩️ `change-storey-bracing-length` inverse: restores the previous `bracing_length`.

use crate::change_storey_bracing_length::ChangeStoreyBracingLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyBracingLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_bracing_length::ChangeStoreyBracingLength> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyBracingLength { id: payload.id.clone(), new_bracing_length: row.bracing_length })],
        None => Vec::new(),
    })
}
