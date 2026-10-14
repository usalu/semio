use crate::change_fire_rei::ChangeFireRei;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeFireRei, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_rei::ChangeFireRei> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeFireRei { index: payload.index, new_fire_rei_min: base.walls[payload.index].fire_rei_min })] }

    })())
}
