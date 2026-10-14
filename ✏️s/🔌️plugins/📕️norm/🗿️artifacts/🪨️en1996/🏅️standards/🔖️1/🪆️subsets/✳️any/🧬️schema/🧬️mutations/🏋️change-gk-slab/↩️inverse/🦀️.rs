use crate::change_gk_slab::ChangeGKSlab;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeGKSlab, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_gk_slab::ChangeGKSlab> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![Op::from(ChangeGKSlab {
            wall_index: payload.wall_index,
            index: payload.index,
            new_g_k_slab_n: base.walls[payload.wall_index].load_cases[payload.index].g_k_slab_n,
        })]
    }

    })())
}
