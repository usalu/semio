//! ↩️ `change-load-case-horiz-variable` inverse: restores the previous `horizontal_variable`.

use crate::change_load_case_horiz_variable::ChangeLoadCaseHorizVariable;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadCaseHorizVariable, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_case_horiz_variable::ChangeLoadCaseHorizVariable> {
    Ok(match base.footings.iter().find(|row| row.id == payload.footing_id).and_then(|row| row.load_cases.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeLoadCaseHorizVariable { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_horizontal_variable: row.horizontal_variable })],
        None => Vec::new(),
    })
}
