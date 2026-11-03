//! ↩️ `change-cover` inverse: restores the previous `cover_m`.

use crate::change_cover::ChangeCover;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeCover, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cover::ChangeCover> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCover, base: &En1996Snapshot) -> Option<ChangeCover> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCover { id: payload.id.clone(), new_cover_m: r0.cover_m })
}
