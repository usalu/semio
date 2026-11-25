use crate::remove_truss::RemoveTruss;
use crate::insert_truss;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveTruss, base: &En1995Snapshot) -> Option<crate::insert_truss::InsertTruss> {
    match base.trusses.get(payload.index) {
        Some(item) => Some(insert_truss::InsertTruss { index: payload.index, truss: item.clone() }),
        None => None,
    }
}
