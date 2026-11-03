//! ↩️ `change-load-case-name` inverse: restores the previous `name`.

use crate::change_load_case_name::ChangeLoadCaseName;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadCaseName, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_case_name::ChangeLoadCaseName> {
    Ok(match base.load_cases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLoadCaseName { id: payload.id.clone(), new_name: row.name.clone() })],
        None => Vec::new(),
    })
}
