//! ↩️ `change-crane-crane-nu` inverse: restores the previous `crane_nu`.

use crate::change_crane_crane_nu::ChangeCraneCraneNu;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneCraneNu, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_crane_nu::ChangeCraneCraneNu> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneCraneNu { id: payload.id.clone(), new_crane_nu: row.crane_nu })],
        None => Vec::new(),
    })
}
