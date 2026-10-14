use crate::change_c_pe::ChangeCPe;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeCPe, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_c_pe::ChangeCPe> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![Op::from(ChangeCPe {
            wall_index: payload.wall_index,
            index: payload.index,
            new_c_pe: base.walls[payload.wall_index].load_cases[payload.index].c_pe,
        })]
    }

    })())
}
