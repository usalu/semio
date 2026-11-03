//! ↩️ `change-bar-kind` inverse: restores the previous `kind`.

use crate::change_bar_kind::ChangeBarKind;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeBarKind, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_kind::ChangeBarKind> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarKind, base: &En1996Snapshot) -> Option<ChangeBarKind> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.bars.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBarKind { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_kind: r1.kind.clone() })
}
