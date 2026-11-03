//! ↩️ `change-slab-bearing-depth` inverse: restores the previous `slab_bearing_depth_m`.

use crate::change_slab_bearing_depth::ChangeSlabBearingDepth;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeSlabBearingDepth, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_bearing_depth::ChangeSlabBearingDepth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSlabBearingDepth, base: &En1996Snapshot) -> Option<ChangeSlabBearingDepth> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSlabBearingDepth { id: payload.id.clone(), new_slab_bearing_depth_m: r0.slab_bearing_depth_m })
}
