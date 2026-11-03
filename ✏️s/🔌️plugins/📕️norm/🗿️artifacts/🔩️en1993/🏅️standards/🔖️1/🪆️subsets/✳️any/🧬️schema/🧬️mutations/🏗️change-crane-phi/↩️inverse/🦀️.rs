//! ↩️ `change-crane-phi` inverse: restores the previous `phi`.

use crate::change_crane_phi::ChangeCranePhi;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCranePhi, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_phi::ChangeCranePhi> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCranePhi { id: payload.id.clone(), new_phi: row.phi })],
        None => Vec::new(),
    })
}
