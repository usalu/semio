use crate::remove_fatigue_band::RemoveFatigueBand;
use crate::insert_fatigue_band;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &RemoveFatigueBand, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fatigue_band::InsertFatigueBand> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.id).and_then(|row| row.spectrum.get(payload.index)) {
        Some(element) => vec![Op::from(insert_fatigue_band::InsertFatigueBand { id: payload.id.clone(), index: Some(payload.index), element: element.clone() })],
        None => Vec::new(),
    })
}
