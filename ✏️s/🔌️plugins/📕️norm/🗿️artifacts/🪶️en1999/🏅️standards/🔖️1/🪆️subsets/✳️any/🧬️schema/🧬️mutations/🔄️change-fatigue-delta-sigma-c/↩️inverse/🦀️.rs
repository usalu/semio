//! ↩️ `change-fatigue-delta-sigma-c` inverse: restores the previous `delta_sigma_c`.

use crate::change_fatigue_delta_sigma_c::ChangeFatigueDeltaSigmaC;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueDeltaSigmaC, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_delta_sigma_c::ChangeFatigueDeltaSigmaC> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.fatigue_detail_id) {
        Some(row) => vec![Op::from(ChangeFatigueDeltaSigmaC { fatigue_detail_id: payload.fatigue_detail_id.clone(), new_delta_sigma_c: row.delta_sigma_c })],
        None => Vec::new(),
    })
}
