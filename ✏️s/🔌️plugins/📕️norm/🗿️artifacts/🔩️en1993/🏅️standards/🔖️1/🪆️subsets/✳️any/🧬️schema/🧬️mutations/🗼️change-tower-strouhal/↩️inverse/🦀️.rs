//! ↩️ `change-tower-strouhal` inverse: restores the previous `strouhal`.

use crate::change_tower_strouhal::ChangeTowerStrouhal;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerStrouhal, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_strouhal::ChangeTowerStrouhal> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerStrouhal { id: payload.id.clone(), new_strouhal: row.strouhal })],
        None => Vec::new(),
    })
}
