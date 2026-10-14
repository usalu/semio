use crate::change_hk_earth::ChangeHKEarth;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeHKEarth, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hk_earth::ChangeHKEarth> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![Op::from(ChangeHKEarth {
            wall_index: payload.wall_index,
            index: payload.index,
            new_h_k_earth_n: base.walls[payload.wall_index].load_cases[payload.index].h_k_earth_n,
        })]
    }

    })())
}
