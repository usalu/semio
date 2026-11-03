use crate::insert_retaining_wall::InsertRetainingWall;
use crate::remove_retaining_wall::RemoveRetainingWall;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertRetainingWall, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_retaining_wall::RemoveRetainingWall> {
    Ok((|| {
    let at = payload.index.min(base.retaining_walls.len());
    vec![Op::from(RemoveRetainingWall { index: at })]

    })())
}
