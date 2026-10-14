use crate::change_opening_sill::ChangeOpeningSill;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeOpeningSill, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_opening_sill::ChangeOpeningSill> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].openings.len() { Vec::new() } else { vec![Op::from(ChangeOpeningSill { wall_index: payload.wall_index, index: payload.index, new_sill_height_m: base.walls[payload.wall_index].openings[payload.index].sill_height_m })] }

    })())
}
