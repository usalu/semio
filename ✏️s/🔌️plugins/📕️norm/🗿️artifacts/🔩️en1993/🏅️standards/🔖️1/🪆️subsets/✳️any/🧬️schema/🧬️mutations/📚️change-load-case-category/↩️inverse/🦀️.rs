//! ↩️ `change-load-case-category` inverse: restores the previous `category`.

use crate::change_load_case_category::ChangeLoadCaseCategory;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadCaseCategory, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_case_category::ChangeLoadCaseCategory> {
    Ok(match base.load_cases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLoadCaseCategory { id: payload.id.clone(), new_category: row.category.clone() })],
        None => Vec::new(),
    })
}
