//! ↩️ `change-reinforced` inverse: restores the previous `reinforced`.

use crate::change_reinforced::ChangeReinforced;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeReinforced, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reinforced::ChangeReinforced> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReinforced, base: &En1996Snapshot) -> Option<ChangeReinforced> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeReinforced { id: payload.id.clone(), new_reinforced: r0.reinforced })
}
