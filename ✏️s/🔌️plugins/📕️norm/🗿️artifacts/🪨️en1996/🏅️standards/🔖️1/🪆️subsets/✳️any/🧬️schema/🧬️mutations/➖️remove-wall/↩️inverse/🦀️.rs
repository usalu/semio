use super::RemoveWall;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
use crate::mutations::insert_wall;
pub fn inverse(payload: &RemoveWall, base: &En1996Snapshot) -> Result<Vec<En1996Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::InsertWall(insert_wall::InsertWall { index: payload.index, wall: base.walls[payload.index].clone() })] }

    })())
}
