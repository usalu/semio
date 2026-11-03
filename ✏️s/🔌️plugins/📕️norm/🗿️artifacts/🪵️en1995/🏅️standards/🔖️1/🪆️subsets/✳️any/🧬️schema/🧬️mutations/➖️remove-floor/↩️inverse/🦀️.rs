use crate::remove_floor::RemoveFloor;
use crate::insert_floor;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveFloor, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_floor::InsertFloor> {
    Ok(match base.floors.get(payload.index) {
        Some(item) => vec![Op::from(insert_floor::InsertFloor { index: payload.index, floor: item.clone() })],
        None => Vec::new(),
    })
}
