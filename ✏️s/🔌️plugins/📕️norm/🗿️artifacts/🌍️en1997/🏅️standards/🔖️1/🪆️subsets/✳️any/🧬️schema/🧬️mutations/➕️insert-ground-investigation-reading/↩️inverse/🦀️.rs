use crate::insert_ground_investigation_reading::InsertGroundInvestigationReading;
use crate::remove_ground_investigation_reading::RemoveGroundInvestigationReading;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertGroundInvestigationReading, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_ground_investigation_reading::RemoveGroundInvestigationReading> {
    let at = base.investigations.iter().find(|row| row.id == payload.investigation_id).map_or(0, |parent| payload.index.min(parent.readings.len()));
    Ok(vec![Op::from(RemoveGroundInvestigationReading { investigation_id: payload.investigation_id.clone(), index: at })])
}
