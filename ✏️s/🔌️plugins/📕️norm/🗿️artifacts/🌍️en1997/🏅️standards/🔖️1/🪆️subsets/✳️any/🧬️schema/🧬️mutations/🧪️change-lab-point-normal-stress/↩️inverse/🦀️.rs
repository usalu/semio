//! ↩️ `change-lab-point-normal-stress` inverse: restores the previous `normal_stress`.

use crate::change_lab_point_normal_stress::ChangeLabPointNormalStress;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabPointNormalStress, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_point_normal_stress::ChangeLabPointNormalStress> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.lab_test_id).and_then(|row| row.points.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeLabPointNormalStress { lab_test_id: payload.lab_test_id.clone(), id: payload.id.clone(), new_normal_stress: row.normal_stress })],
        None => Vec::new(),
    })
}
