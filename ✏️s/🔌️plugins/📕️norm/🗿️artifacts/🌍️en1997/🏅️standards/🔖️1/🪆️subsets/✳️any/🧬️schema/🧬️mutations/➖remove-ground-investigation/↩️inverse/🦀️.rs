use crate::remove_ground_investigation::RemoveGroundInvestigation;
use crate::insert_ground_investigation::InsertGroundInvestigation;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveGroundInvestigation, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_ground_investigation::InsertGroundInvestigation> {
    Ok((|| {
    match base.investigations.get(payload.index).cloned() {
        Some(investigation) => vec![Op::from(InsertGroundInvestigation { index: payload.index, investigation })],
        None => Vec::new(),
    }

    })())
}
