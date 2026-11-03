//! ↩️ `change-lab-test-consol-time` inverse: restores the previous `consolidation_time`.

use crate::change_lab_test_consol_time::ChangeLabTestConsolTime;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabTestConsolTime, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_test_consol_time::ChangeLabTestConsolTime> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLabTestConsolTime { id: payload.id.clone(), new_consolidation_time: row.consolidation_time })],
        None => Vec::new(),
    })
}
