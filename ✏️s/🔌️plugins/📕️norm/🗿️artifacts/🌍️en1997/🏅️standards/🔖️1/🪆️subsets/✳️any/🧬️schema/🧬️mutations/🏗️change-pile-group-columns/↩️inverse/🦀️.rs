//! ↩️ `change-pile-group-columns` inverse: restores the previous `columns`.

use crate::change_pile_group_columns::ChangePileGroupColumns;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileGroupColumns, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_group_columns::ChangePileGroupColumns> {
    Ok(match base.pile_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileGroupColumns { id: payload.id.clone(), new_columns: row.columns })],
        None => Vec::new(),
    })
}
