use crate::change_wall_type::ChangeWallType;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeWallType, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_type::ChangeWallType> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeWallType { index: payload.index, new_wall_type: base.walls[payload.index].wall_type })] }

    })())
}
