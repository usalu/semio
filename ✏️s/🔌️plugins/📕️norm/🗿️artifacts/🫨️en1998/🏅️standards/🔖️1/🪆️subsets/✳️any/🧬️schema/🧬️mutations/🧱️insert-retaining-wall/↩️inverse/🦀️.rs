//! Inverse for `insert-retaining-wall`.
use crate::insert_retaining_wall::InsertRetainingWall;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::remove_retaining_wall;

pub fn inverse<Op>(payload: &InsertRetainingWall, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_retaining_wall::RemoveRetainingWall> {
    Ok((|| {
    vec![Op::from(remove_retaining_wall::RemoveRetainingWall { index: payload.index.unwrap_or(usize::MAX).min(base.retaining_walls.len()) })]

    })())
}
