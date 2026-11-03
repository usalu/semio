//! ↩️ `change-load-case-vertical-var` inverse: restores the previous `vertical_variable`.

use crate::change_load_case_vertical_var::ChangeLoadCaseVerticalVar;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadCaseVerticalVar, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_case_vertical_var::ChangeLoadCaseVerticalVar> {
    Ok(match base.footings.iter().find(|row| row.id == payload.footing_id).and_then(|row| row.load_cases.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeLoadCaseVerticalVar { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_vertical_variable: row.vertical_variable })],
        None => Vec::new(),
    })
}
