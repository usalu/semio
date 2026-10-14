use crate::remove_opening::RemoveOpening;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use crate::insert_opening;
pub fn inverse<Op>(payload: &RemoveOpening, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_opening::InsertOpening> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].openings.len() { Vec::new() } else { vec![Op::from(insert_opening::InsertOpening { wall_index: payload.wall_index, index: payload.index, opening: base.walls[payload.wall_index].openings[payload.index].clone() })] }

    })())
}
