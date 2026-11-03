//! ↩️ `change-support-sides` inverse: restores the previous `support_sides`.

use crate::change_support_sides::ChangeSupportSides;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeSupportSides, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_support_sides::ChangeSupportSides> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSupportSides, base: &En1996Snapshot) -> Option<ChangeSupportSides> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSupportSides { id: payload.id.clone(), new_support_sides: r0.support_sides })
}
