//! ↩️ `change-mu` inverse: restores the previous `mu`.

use crate::change_mu::ChangeMu;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeMu, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_mu::ChangeMu> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMu, base: &En1996Snapshot) -> Option<ChangeMu> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMu { id: payload.id.clone(), new_mu: r0.mu })
}
