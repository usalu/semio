//! ↩️ `change-diaphragm-depth-m` inverse: restores the previous `depth_m`.

use crate::change_diaphragm_depth_m::ChangeDiaphragmDepthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeDiaphragmDepthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_depth_m::ChangeDiaphragmDepthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDiaphragmDepthM, base: &En1998Snapshot) -> Option<ChangeDiaphragmDepthM> {
    let r0 = base.diaphragms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDiaphragmDepthM { id: payload.id.clone(), new_depth_m: r0.depth_m })
}
