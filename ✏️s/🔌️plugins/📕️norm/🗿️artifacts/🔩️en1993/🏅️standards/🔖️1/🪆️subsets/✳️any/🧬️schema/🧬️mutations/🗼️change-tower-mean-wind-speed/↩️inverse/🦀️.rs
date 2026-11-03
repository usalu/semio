//! ↩️ `change-tower-mean-wind-speed` inverse: restores the previous `mean_wind_speed`.

use crate::change_tower_mean_wind_speed::ChangeTowerMeanWindSpeed;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerMeanWindSpeed, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_mean_wind_speed::ChangeTowerMeanWindSpeed> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerMeanWindSpeed { id: payload.id.clone(), new_mean_wind_speed: row.mean_wind_speed })],
        None => Vec::new(),
    })
}
