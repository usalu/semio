//! ↩️ `change-storey-columns` inverse: restores the previous `columns`.

use crate::change_storey_columns::ChangeStoreyColumns;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyColumns, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_columns::ChangeStoreyColumns> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyColumns { id: payload.id.clone(), new_columns: row.columns })],
        None => Vec::new(),
    })
}
