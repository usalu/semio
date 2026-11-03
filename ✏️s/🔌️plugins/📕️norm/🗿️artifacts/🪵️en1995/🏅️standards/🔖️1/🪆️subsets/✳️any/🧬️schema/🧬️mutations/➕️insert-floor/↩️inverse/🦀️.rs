use crate::insert_floor::InsertFloor;
use crate::remove_floor;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertFloor, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_floor::RemoveFloor> {
    let at = payload.index.min(base.floors.len());
    Ok(vec![Op::from(remove_floor::RemoveFloor { index: at })])
}
