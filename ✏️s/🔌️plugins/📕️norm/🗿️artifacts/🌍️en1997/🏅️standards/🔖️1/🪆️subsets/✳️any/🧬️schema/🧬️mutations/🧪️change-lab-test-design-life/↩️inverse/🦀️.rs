//! ↩️ `change-lab-test-design-life` inverse: restores the previous `design_life`.

use crate::change_lab_test_design_life::ChangeLabTestDesignLife;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLabTestDesignLife, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_lab_test_design_life::ChangeLabTestDesignLife> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLabTestDesignLife { id: payload.id.clone(), new_design_life: row.design_life })],
        None => Vec::new(),
    })
}
