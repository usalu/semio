//! ↩️ `change-load-case-kind` inverse: restores the previous `kind`.

use crate::change_load_case_kind::ChangeLoadCaseKind;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadCaseKind, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_case_kind::ChangeLoadCaseKind> {
    Ok(match base.load_cases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLoadCaseKind { id: payload.id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
