//! ↩️ `change-weld-width` inverse: restores the previous `width`.

use crate::change_weld_width::ChangeWeldWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_width::ChangeWeldWidth> {
    Ok(match base.weld_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWeldWidth { id: payload.id.clone(), new_width: row.width })],
        None => Vec::new(),
    })
}
