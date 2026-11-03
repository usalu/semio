//! ↩️ `change-lab-test-preload-stress` inverse: restores the previous `preload_stress`.

use crate::change_lab_test_preload_stress::ChangeLabTestPreloadStress;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabTestPreloadStress, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_test_preload_stress::ChangeLabTestPreloadStress> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLabTestPreloadStress { id: payload.id.clone(), new_preload_stress: row.preload_stress })],
        None => Vec::new(),
    })
}
