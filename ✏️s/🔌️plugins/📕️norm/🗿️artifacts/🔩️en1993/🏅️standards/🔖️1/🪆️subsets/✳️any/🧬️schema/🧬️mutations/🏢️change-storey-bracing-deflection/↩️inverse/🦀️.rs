//! ↩️ `change-storey-bracing-deflection` inverse: restores the previous `bracing_deflection`.

use crate::change_storey_bracing_deflection::ChangeStoreyBracingDeflection;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyBracingDeflection, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_bracing_deflection::ChangeStoreyBracingDeflection> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyBracingDeflection { id: payload.id.clone(), new_bracing_deflection: row.bracing_deflection })],
        None => Vec::new(),
    })
}
