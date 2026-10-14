use crate::change_opening_height::ChangeOpeningHeight;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeOpeningHeight, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_opening_height::ChangeOpeningHeight> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].openings.len() { Vec::new() } else { vec![Op::from(ChangeOpeningHeight { wall_index: payload.wall_index, index: payload.index, new_height_m: base.walls[payload.wall_index].openings[payload.index].height_m })] }

    })())
}
