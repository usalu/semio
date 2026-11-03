//! ↩️ `change-weld-depth` inverse: restores the previous `depth`.

use crate::change_weld_depth::ChangeWeldDepth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldDepth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_depth::ChangeWeldDepth> {
    Ok(match base.weld_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWeldDepth { id: payload.id.clone(), new_depth: row.depth })],
        None => Vec::new(),
    })
}
