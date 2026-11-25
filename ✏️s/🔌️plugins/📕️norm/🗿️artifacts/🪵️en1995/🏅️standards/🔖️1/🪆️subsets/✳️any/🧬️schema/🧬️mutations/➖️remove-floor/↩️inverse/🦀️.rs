use crate::remove_floor::RemoveFloor;
use crate::insert_floor;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveFloor, base: &En1995Snapshot) -> Option<crate::insert_floor::InsertFloor> {
    match base.floors.get(payload.index) {
        Some(item) => Some(insert_floor::InsertFloor { index: payload.index, floor: item.clone() }),
        None => None,
    }
}
