use crate::remove_truss::RemoveTruss;
use crate::insert_truss;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveTruss, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_truss::InsertTruss> {
    Ok(match base.trusses.get(payload.index) {
        Some(item) => vec![Op::from(insert_truss::InsertTruss { index: payload.index, truss: item.clone() })],
        None => Vec::new(),
    })
}
