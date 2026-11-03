use crate::remove_diaphragm::RemoveDiaphragm;
use crate::insert_diaphragm;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveDiaphragm, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_diaphragm::InsertDiaphragm> {
    Ok(match base.diaphragms.get(payload.index) {
        Some(item) => vec![Op::from(insert_diaphragm::InsertDiaphragm { index: payload.index, diaphragm: item.clone() })],
        None => Vec::new(),
    })
}
