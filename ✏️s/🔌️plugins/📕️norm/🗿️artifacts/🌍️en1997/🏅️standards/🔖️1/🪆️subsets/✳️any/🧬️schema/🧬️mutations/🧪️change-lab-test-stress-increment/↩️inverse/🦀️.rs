//! ↩️ `change-lab-test-stress-increment` inverse: restores the previous `stress_increment`.

use crate::change_lab_test_stress_increment::ChangeLabTestStressIncrement;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabTestStressIncrement, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_test_stress_increment::ChangeLabTestStressIncrement> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLabTestStressIncrement { id: payload.id.clone(), new_stress_increment: row.stress_increment })],
        None => Vec::new(),
    })
}
