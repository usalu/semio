use crate::change_qk_snow::ChangeQKSnow;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeQKSnow, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_qk_snow::ChangeQKSnow> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![Op::from(ChangeQKSnow {
            wall_index: payload.wall_index,
            index: payload.index,
            new_q_k_snow_pa: base.walls[payload.wall_index].load_cases[payload.index].q_k_snow_pa,
        })]
    }

    })())
}
