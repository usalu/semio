use crate::insert_diaphragm::InsertDiaphragm;
use crate::remove_diaphragm;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertDiaphragm, base: &En1995Snapshot) -> Option<crate::remove_diaphragm::RemoveDiaphragm> {
    let at = payload.index.min(base.diaphragms.len());
    Some(remove_diaphragm::RemoveDiaphragm { index: at })
}
