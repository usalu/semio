//! ↩️ `change-wall-concrete-gamma` inverse: restores the previous `concrete_gamma`.

use crate::change_wall_concrete_gamma::ChangeWallConcreteGamma;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallConcreteGamma, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_concrete_gamma::ChangeWallConcreteGamma> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallConcreteGamma { id: payload.id.clone(), new_concrete_gamma: row.concrete_gamma })],
        None => Vec::new(),
    })
}
