//! ↩️ `change-bracing-system` inverse: restores the previous `bracing_system`.

use crate::change_bracing_system::ChangeBracingSystem;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeBracingSystem, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bracing_system::ChangeBracingSystem> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBracingSystem, base: &En1996Snapshot) -> Option<ChangeBracingSystem> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBracingSystem { id: payload.id.clone(), new_bracing_system: r0.bracing_system })
}
