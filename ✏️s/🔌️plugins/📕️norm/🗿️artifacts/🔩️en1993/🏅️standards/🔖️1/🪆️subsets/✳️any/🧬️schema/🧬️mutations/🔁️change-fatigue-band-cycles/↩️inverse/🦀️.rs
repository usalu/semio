//! ↩️ `change-fatigue-band-cycles` inverse: restores the previous `cycles`.

use crate::change_fatigue_band_cycles::ChangeFatigueBandCycles;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueBandCycles, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_band_cycles::ChangeFatigueBandCycles> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.id).and_then(|row| row.spectrum.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeFatigueBandCycles { id: payload.id.clone(), element_id: payload.element_id.clone(), new_cycles: row.cycles })],
        None => Vec::new(),
    })
}
