use crate::change_qp_wind::ChangeQPWind;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeQPWind, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_qp_wind::ChangeQPWind> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![Op::from(ChangeQPWind {
            wall_index: payload.wall_index,
            index: payload.index,
            new_q_p_wind_pa: base.walls[payload.wall_index].load_cases[payload.index].q_p_wind_pa,
        })]
    }

    })())
}
