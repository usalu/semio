//! ↩️ `change-crane-fy` inverse: restores the previous `fy`.

use crate::change_crane_fy::ChangeCraneFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_fy::ChangeCraneFy> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneFy { id: payload.id.clone(), new_fy: row.fy })],
        None => Vec::new(),
    })
}
