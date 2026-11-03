//! ↩️ `change-lab-test-layer-id` inverse: restores the previous `layer_id`.

use crate::change_lab_test_layer_id::ChangeLabTestLayerId;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabTestLayerId, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_test_layer_id::ChangeLabTestLayerId> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLabTestLayerId { id: payload.id.clone(), new_layer_id: row.layer_id.clone() })],
        None => Vec::new(),
    })
}
