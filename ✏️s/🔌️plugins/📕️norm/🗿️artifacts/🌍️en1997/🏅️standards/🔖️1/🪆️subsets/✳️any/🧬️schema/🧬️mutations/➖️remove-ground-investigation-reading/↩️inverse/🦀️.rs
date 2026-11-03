use crate::remove_ground_investigation_reading::RemoveGroundInvestigationReading;
use crate::insert_ground_investigation_reading::InsertGroundInvestigationReading;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveGroundInvestigationReading, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_ground_investigation_reading::InsertGroundInvestigationReading> {
    Ok(match base.investigations.iter().find(|row| row.id == payload.investigation_id).and_then(|parent| parent.readings.get(payload.index).cloned()) {
        Some(investigation_reading) => vec![Op::from(InsertGroundInvestigationReading { investigation_id: payload.investigation_id.clone(), index: payload.index, investigation_reading })],
        None => Vec::new(),
    })
}
