//! ↩️ `change-tower-height` inverse: restores the previous `height`.

use crate::change_tower_height::ChangeTowerHeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerHeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_height::ChangeTowerHeight> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerHeight { id: payload.id.clone(), new_height: row.height })],
        None => Vec::new(),
    })
}
