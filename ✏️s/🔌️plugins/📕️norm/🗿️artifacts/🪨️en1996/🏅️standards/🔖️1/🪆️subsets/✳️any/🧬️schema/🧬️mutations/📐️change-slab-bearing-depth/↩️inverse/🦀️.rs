use crate::change_slab_bearing_depth::ChangeSlabBearingDepth;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabBearingDepth, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_bearing_depth::ChangeSlabBearingDepth> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeSlabBearingDepth { index: payload.index, new_slab_bearing_depth_m: base.walls[payload.index].slab_bearing_depth_m })] }

    })())
}
