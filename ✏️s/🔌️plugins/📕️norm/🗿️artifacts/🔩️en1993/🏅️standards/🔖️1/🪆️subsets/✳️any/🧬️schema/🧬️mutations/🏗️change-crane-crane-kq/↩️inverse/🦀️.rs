//! ↩️ `change-crane-crane-kq` inverse: restores the previous `crane_kq`.

use crate::change_crane_crane_kq::ChangeCraneCraneKq;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneCraneKq, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_crane_kq::ChangeCraneCraneKq> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneCraneKq { id: payload.id.clone(), new_crane_kq: row.crane_kq })],
        None => Vec::new(),
    })
}
