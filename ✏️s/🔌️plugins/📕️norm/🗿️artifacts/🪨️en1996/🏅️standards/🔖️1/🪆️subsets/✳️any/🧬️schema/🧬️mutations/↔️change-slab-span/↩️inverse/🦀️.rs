use crate::change_slab_span::ChangeSlabSpan;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabSpan, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_span::ChangeSlabSpan> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![Op::from(ChangeSlabSpan {
            wall_index: payload.wall_index,
            index: payload.index,
            new_slab_span_m: base.walls[payload.wall_index].load_cases[payload.index].slab_span_m,
        })]
    }

    })())
}
