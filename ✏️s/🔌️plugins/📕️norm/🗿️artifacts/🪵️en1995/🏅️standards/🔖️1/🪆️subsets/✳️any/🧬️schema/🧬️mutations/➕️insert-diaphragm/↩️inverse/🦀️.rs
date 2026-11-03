use crate::insert_diaphragm::InsertDiaphragm;
use crate::remove_diaphragm;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertDiaphragm, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_diaphragm::RemoveDiaphragm> {
    let at = payload.index.min(base.diaphragms.len());
    Ok(vec![Op::from(remove_diaphragm::RemoveDiaphragm { index: at })])
}
