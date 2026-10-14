use crate::change_wall_thickness::ChangeWallThickness;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeWallThickness, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_thickness::ChangeWallThickness> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeWallThickness { index: payload.index, new_thickness_m: base.walls[payload.index].thickness_m })] }

    })())
}
