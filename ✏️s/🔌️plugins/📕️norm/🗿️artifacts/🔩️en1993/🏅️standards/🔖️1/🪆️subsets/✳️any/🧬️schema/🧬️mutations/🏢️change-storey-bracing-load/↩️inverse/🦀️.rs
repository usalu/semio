//! ↩️ `change-storey-bracing-load` inverse: restores the previous `bracing_load`.

use crate::change_storey_bracing_load::ChangeStoreyBracingLoad;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyBracingLoad, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_bracing_load::ChangeStoreyBracingLoad> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyBracingLoad { id: payload.id.clone(), new_bracing_load: row.bracing_load })],
        None => Vec::new(),
    })
}
