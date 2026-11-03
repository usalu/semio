//! ↩️ `change-storey-horizontal-load` inverse: restores the previous `horizontal_load`.

use crate::change_storey_horizontal_load::ChangeStoreyHorizontalLoad;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyHorizontalLoad, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_horizontal_load::ChangeStoreyHorizontalLoad> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyHorizontalLoad { id: payload.id.clone(), new_horizontal_load: row.horizontal_load })],
        None => Vec::new(),
    })
}
