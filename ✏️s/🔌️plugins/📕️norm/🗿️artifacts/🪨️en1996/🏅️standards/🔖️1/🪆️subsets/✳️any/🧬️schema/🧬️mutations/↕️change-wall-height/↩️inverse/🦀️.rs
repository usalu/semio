use crate::change_wall_height::ChangeWallHeight;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeWallHeight, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_height::ChangeWallHeight> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeWallHeight { index: payload.index, new_height_m: base.walls[payload.index].height_m })] }

    })())
}
