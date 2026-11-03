use crate::remove_retaining_wall::RemoveRetainingWall;
use crate::insert_retaining_wall::InsertRetainingWall;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveRetainingWall, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_retaining_wall::InsertRetainingWall> {
    Ok((|| {
    match base.retaining_walls.get(payload.index).cloned() {
        Some(retaining_wall) => vec![Op::from(InsertRetainingWall { index: payload.index, retaining_wall })],
        None => Vec::new(),
    }

    })())
}
