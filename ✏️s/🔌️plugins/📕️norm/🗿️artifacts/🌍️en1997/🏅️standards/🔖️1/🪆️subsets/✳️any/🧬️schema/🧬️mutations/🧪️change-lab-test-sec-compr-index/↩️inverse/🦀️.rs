//! ↩️ `change-lab-test-sec-compr-index` inverse: restores the previous `secondary_compression_index`.

use crate::change_lab_test_sec_compr_index::ChangeLabTestSecComprIndex;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabTestSecComprIndex, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_test_sec_compr_index::ChangeLabTestSecComprIndex> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLabTestSecComprIndex { id: payload.id.clone(), new_secondary_compression_index: row.secondary_compression_index })],
        None => Vec::new(),
    })
}
