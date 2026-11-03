//! ↩️ `change-weld-throat` inverse: restores the previous `throat`.

use crate::change_weld_throat::ChangeWeldThroat;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldThroat, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_throat::ChangeWeldThroat> {
    Ok(match base.weld_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWeldThroat { id: payload.id.clone(), new_throat: row.throat })],
        None => Vec::new(),
    })
}
