//! ↩️ `change-load-case-moment-variable` inverse: restores the previous `moment_variable`.

use crate::change_load_case_moment_variable::ChangeLoadCaseMomentVariable;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadCaseMomentVariable, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_case_moment_variable::ChangeLoadCaseMomentVariable> {
    Ok(match base.footings.iter().find(|row| row.id == payload.footing_id).and_then(|row| row.load_cases.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeLoadCaseMomentVariable { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_moment_variable: row.moment_variable })],
        None => Vec::new(),
    })
}
