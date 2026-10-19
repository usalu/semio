//! ↩️ Inverse for `remove-retaining-wall`.
use crate::remove_retaining_wall::RemoveRetainingWall;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::insert_retaining_wall;

pub fn inverse<Op>(payload: &RemoveRetainingWall, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_retaining_wall::InsertRetainingWall> {
    Ok((|| {
    match base.retaining_walls.get(payload.index) {
        Some(item) => vec![Op::from(insert_retaining_wall::InsertRetainingWall { index: Some(payload.index), wall: item.clone() })],
        None => Vec::new(),
    }

    })())
}
