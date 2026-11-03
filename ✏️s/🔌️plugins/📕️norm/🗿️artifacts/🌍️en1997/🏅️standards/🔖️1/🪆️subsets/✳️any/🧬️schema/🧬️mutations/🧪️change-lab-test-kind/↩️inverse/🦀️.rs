//! ↩️ `change-lab-test-kind` inverse: restores the previous `kind`.

use crate::change_lab_test_kind::ChangeLabTestKind;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabTestKind, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_test_kind::ChangeLabTestKind> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLabTestKind { id: payload.id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
