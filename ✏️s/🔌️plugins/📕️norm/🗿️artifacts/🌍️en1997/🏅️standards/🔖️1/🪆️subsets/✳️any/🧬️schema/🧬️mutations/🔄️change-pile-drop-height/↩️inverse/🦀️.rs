//! ↩️ `change-pile-drop-height` inverse: restores the previous `drop_height`.

use crate::change_pile_drop_height::ChangePileDropHeight;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileDropHeight, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_drop_height::ChangePileDropHeight> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileDropHeight { id: payload.id.clone(), new_drop_height: row.drop_height })],
        None => Vec::new(),
    })
}
