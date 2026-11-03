//! ↩️ `change-chase-depth` inverse: restores the previous `depth_m`.

use crate::change_chase_depth::ChangeChaseDepth;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeChaseDepth, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_chase_depth::ChangeChaseDepth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeChaseDepth, base: &En1996Snapshot) -> Option<ChangeChaseDepth> {
    let r0 = base.chases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeChaseDepth { id: payload.id.clone(), new_depth_m: r0.depth_m })
}
