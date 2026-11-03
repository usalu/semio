//! ↩️ `change-lab-test-swelling-index` inverse: restores the previous `swelling_index`.

use crate::change_lab_test_swelling_index::ChangeLabTestSwellingIndex;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabTestSwellingIndex, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_test_swelling_index::ChangeLabTestSwellingIndex> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLabTestSwellingIndex { id: payload.id.clone(), new_swelling_index: row.swelling_index })],
        None => Vec::new(),
    })
}
