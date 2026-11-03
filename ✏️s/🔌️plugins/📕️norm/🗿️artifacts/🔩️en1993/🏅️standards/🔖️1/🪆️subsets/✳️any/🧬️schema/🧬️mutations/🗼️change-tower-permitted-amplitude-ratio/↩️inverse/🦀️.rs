//! ↩️ `change-tower-permitted-amplitude-ratio` inverse: restores the previous `permitted_amplitude_ratio`.

use crate::change_tower_permitted_amplitude_ratio::ChangeTowerPermittedAmplitudeRatio;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerPermittedAmplitudeRatio, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_permitted_amplitude_ratio::ChangeTowerPermittedAmplitudeRatio> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerPermittedAmplitudeRatio { id: payload.id.clone(), new_permitted_amplitude_ratio: row.permitted_amplitude_ratio })],
        None => Vec::new(),
    })
}
