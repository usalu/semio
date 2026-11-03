//! ↩️ `change-material-grade` inverse: restores the previous `grade`.

use crate::change_material_grade::ChangeMaterialGrade;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialGrade, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_grade::ChangeMaterialGrade> {
    Ok(match base.materials.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMaterialGrade { id: payload.id.clone(), new_grade: row.grade.clone() })],
        None => Vec::new(),
    })
}
