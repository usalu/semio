use crate::insert_floor::InsertFloor;
use crate::remove_floor;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertFloor, base: &En1995Snapshot) -> Option<crate::remove_floor::RemoveFloor> {
    let at = payload.index.min(base.floors.len());
    Some(remove_floor::RemoveFloor { index: at })
}
