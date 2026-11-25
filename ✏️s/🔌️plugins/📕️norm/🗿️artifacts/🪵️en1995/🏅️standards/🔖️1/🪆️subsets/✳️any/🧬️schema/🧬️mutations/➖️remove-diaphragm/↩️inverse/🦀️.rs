use crate::remove_diaphragm::RemoveDiaphragm;
use crate::insert_diaphragm;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveDiaphragm, base: &En1995Snapshot) -> Option<crate::insert_diaphragm::InsertDiaphragm> {
    match base.diaphragms.get(payload.index) {
        Some(item) => Some(insert_diaphragm::InsertDiaphragm { index: payload.index, diaphragm: item.clone() }),
        None => None,
    }
}
