use crate::change_wall_length::ChangeWallLength;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeWallLength, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_length::ChangeWallLength> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeWallLength { index: payload.index, new_length_m: base.walls[payload.index].length_m })] }

    })())
}
