//! ↩️ `change-pile-group-rows` inverse: restores the previous `rows`.

use crate::change_pile_group_rows::ChangePileGroupRows;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileGroupRows, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_group_rows::ChangePileGroupRows> {
    Ok(match base.pile_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileGroupRows { id: payload.id.clone(), new_rows: row.rows })],
        None => Vec::new(),
    })
}
