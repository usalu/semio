use crate::insert_truss::InsertTruss;
use crate::remove_truss;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertTruss, base: &En1995Snapshot) -> Option<crate::remove_truss::RemoveTruss> {
    let at = payload.index.min(base.trusses.len());
    Some(remove_truss::RemoveTruss { index: at })
}
