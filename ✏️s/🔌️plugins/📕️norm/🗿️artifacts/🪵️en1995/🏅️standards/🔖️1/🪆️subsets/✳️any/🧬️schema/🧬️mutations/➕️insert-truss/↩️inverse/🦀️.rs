use crate::insert_truss::InsertTruss;
use crate::remove_truss;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertTruss, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_truss::RemoveTruss> {
    let at = payload.index.min(base.trusses.len());
    Ok(vec![Op::from(remove_truss::RemoveTruss { index: at })])
}
