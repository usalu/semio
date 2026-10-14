use crate::remove_wall::RemoveWall;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use crate::insert_wall;
pub fn inverse<Op>(payload: &RemoveWall, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_wall::InsertWall> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(insert_wall::InsertWall { index: payload.index, wall: base.walls[payload.index].clone() })] }

    })())
}
