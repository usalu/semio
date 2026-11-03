//! ↩️ `change-tower-guy-length` inverse: restores the previous `guy_length`.

use crate::change_tower_guy_length::ChangeTowerGuyLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerGuyLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_guy_length::ChangeTowerGuyLength> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerGuyLength { id: payload.id.clone(), new_guy_length: row.guy_length })],
        None => Vec::new(),
    })
}
