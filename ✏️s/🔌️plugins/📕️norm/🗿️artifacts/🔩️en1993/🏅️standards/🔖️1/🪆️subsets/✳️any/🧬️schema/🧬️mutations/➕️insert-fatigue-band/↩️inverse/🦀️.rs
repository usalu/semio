use crate::insert_fatigue_band::InsertFatigueBand;
use crate::remove_fatigue_band;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &InsertFatigueBand, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_fatigue_band::RemoveFatigueBand> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(remove_fatigue_band::RemoveFatigueBand { id: payload.id.clone(), index: payload.index.unwrap_or(usize::MAX).min(row.spectrum.len()) })],
        None => Vec::new(),
    })
}
