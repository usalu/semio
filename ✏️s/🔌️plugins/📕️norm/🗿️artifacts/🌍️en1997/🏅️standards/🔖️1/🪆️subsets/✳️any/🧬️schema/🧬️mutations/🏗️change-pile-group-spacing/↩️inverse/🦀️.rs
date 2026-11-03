//! ↩️ `change-pile-group-spacing` inverse: restores the previous `spacing`.

use crate::change_pile_group_spacing::ChangePileGroupSpacing;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileGroupSpacing, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_group_spacing::ChangePileGroupSpacing> {
    Ok(match base.pile_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileGroupSpacing { id: payload.id.clone(), new_spacing: row.spacing })],
        None => Vec::new(),
    })
}
