//! ↩️ `change-crane-fatigue-phi` inverse: restores the previous `fatigue_phi`.

use crate::change_crane_fatigue_phi::ChangeCraneFatiguePhi;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneFatiguePhi, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_fatigue_phi::ChangeCraneFatiguePhi> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneFatiguePhi { id: payload.id.clone(), new_fatigue_phi: row.fatigue_phi })],
        None => Vec::new(),
    })
}
