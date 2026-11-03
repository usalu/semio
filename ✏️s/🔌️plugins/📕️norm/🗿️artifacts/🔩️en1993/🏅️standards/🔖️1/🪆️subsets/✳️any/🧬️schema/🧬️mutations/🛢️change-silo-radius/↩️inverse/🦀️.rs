//! ↩️ `change-silo-radius` inverse: restores the previous `radius`.

use crate::change_silo_radius::ChangeSiloRadius;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloRadius, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_radius::ChangeSiloRadius> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloRadius { id: payload.id.clone(), new_radius: row.radius })],
        None => Vec::new(),
    })
}
