use crate::insert_ground_investigation::InsertGroundInvestigation;
use crate::remove_ground_investigation::RemoveGroundInvestigation;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertGroundInvestigation, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_ground_investigation::RemoveGroundInvestigation> {
    Ok((|| {
    let at = payload.index.min(base.investigations.len());
    vec![Op::from(RemoveGroundInvestigation { index: at })]

    })())
}
