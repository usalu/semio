//! ↩️ `change-chase-wall` inverse: restores the previous `wall_id`.

use crate::change_chase_wall::ChangeChaseWall;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeChaseWall, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_chase_wall::ChangeChaseWall> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeChaseWall, base: &En1996Snapshot) -> Option<ChangeChaseWall> {
    let r0 = base.chases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeChaseWall { id: payload.id.clone(), new_wall_id: r0.wall_id.clone() })
}
