//! ↩️ `change-lab-point-shear-stress` inverse: restores the previous `shear_stress`.

use crate::change_lab_point_shear_stress::ChangeLabPointShearStress;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabPointShearStress, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_point_shear_stress::ChangeLabPointShearStress> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.lab_test_id).and_then(|row| row.points.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeLabPointShearStress { lab_test_id: payload.lab_test_id.clone(), id: payload.id.clone(), new_shear_stress: row.shear_stress })],
        None => Vec::new(),
    })
}
