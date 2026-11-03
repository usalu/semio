//! ↩️ `change-weld-pattern` inverse: restores the previous `pattern`.

use crate::change_weld_pattern::ChangeWeldPattern;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldPattern, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_pattern::ChangeWeldPattern> {
    Ok(match base.weld_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWeldPattern { id: payload.id.clone(), new_pattern: row.pattern })],
        None => Vec::new(),
    })
}
