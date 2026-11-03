//! ↩️ `change-lab-test-drainage-height` inverse: restores the previous `drainage_height`.

use crate::change_lab_test_drainage_height::ChangeLabTestDrainageHeight;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabTestDrainageHeight, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_test_drainage_height::ChangeLabTestDrainageHeight> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLabTestDrainageHeight { id: payload.id.clone(), new_drainage_height: row.drainage_height })],
        None => Vec::new(),
    })
}
