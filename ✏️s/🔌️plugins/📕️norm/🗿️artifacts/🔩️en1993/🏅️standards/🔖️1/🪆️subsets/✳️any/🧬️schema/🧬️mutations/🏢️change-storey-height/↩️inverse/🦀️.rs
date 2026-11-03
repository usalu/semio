//! ↩️ `change-storey-height` inverse: restores the previous `height`.

use crate::change_storey_height::ChangeStoreyHeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyHeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_height::ChangeStoreyHeight> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyHeight { id: payload.id.clone(), new_height: row.height })],
        None => Vec::new(),
    })
}
