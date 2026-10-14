use crate::insert_wall::InsertWall;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use crate::remove_wall;
pub fn inverse<Op>(payload: &InsertWall, _base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_wall::RemoveWall> {
    Ok((|| {
    vec![Op::from(remove_wall::RemoveWall { index: payload.index })]

    })())
}
