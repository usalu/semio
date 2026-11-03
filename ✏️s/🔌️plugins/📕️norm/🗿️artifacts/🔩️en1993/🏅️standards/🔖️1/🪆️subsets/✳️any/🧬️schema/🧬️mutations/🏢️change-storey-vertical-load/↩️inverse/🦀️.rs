//! ↩️ `change-storey-vertical-load` inverse: restores the previous `vertical_load`.

use crate::change_storey_vertical_load::ChangeStoreyVerticalLoad;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyVerticalLoad, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_vertical_load::ChangeStoreyVerticalLoad> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyVerticalLoad { id: payload.id.clone(), new_vertical_load: row.vertical_load })],
        None => Vec::new(),
    })
}
