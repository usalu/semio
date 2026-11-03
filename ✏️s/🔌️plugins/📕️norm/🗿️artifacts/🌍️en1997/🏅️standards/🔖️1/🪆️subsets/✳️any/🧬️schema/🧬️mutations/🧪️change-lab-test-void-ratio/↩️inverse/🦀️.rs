//! ↩️ `change-lab-test-void-ratio` inverse: restores the previous `void_ratio`.

use crate::change_lab_test_void_ratio::ChangeLabTestVoidRatio;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabTestVoidRatio, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_test_void_ratio::ChangeLabTestVoidRatio> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLabTestVoidRatio { id: payload.id.clone(), new_void_ratio: row.void_ratio })],
        None => Vec::new(),
    })
}
