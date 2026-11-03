//! ↩️ `change-lab-test-compression-index` inverse: restores the previous `compression_index`.

use crate::change_lab_test_compression_index::ChangeLabTestCompressionIndex;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabTestCompressionIndex, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_test_compression_index::ChangeLabTestCompressionIndex> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLabTestCompressionIndex { id: payload.id.clone(), new_compression_index: row.compression_index })],
        None => Vec::new(),
    })
}
