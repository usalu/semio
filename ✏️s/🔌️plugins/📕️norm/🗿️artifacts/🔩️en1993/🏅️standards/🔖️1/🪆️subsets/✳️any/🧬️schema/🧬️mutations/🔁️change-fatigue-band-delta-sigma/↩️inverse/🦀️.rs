//! ↩️ `change-fatigue-band-delta-sigma` inverse: restores the previous `delta_sigma`.

use crate::change_fatigue_band_delta_sigma::ChangeFatigueBandDeltaSigma;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueBandDeltaSigma, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_band_delta_sigma::ChangeFatigueBandDeltaSigma> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.id).and_then(|row| row.spectrum.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeFatigueBandDeltaSigma { id: payload.id.clone(), element_id: payload.element_id.clone(), new_delta_sigma: row.delta_sigma })],
        None => Vec::new(),
    })
}
