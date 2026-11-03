//! ↩️ `change-tower-crossing-rate` inverse: restores the previous `crossing_rate`.

use crate::change_tower_crossing_rate::ChangeTowerCrossingRate;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerCrossingRate, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_crossing_rate::ChangeTowerCrossingRate> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerCrossingRate { id: payload.id.clone(), new_crossing_rate: row.crossing_rate })],
        None => Vec::new(),
    })
}
