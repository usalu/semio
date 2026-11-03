//! ↩️ `change-silo-thickness` inverse: restores the previous `thickness`.

use crate::change_silo_thickness::ChangeSiloThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_thickness::ChangeSiloThickness> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloThickness { id: payload.id.clone(), new_thickness: row.thickness })],
        None => Vec::new(),
    })
}
