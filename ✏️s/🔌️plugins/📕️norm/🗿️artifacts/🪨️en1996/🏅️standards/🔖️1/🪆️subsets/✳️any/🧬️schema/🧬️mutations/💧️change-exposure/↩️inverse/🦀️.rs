//! ↩️ `change-exposure` inverse: restores the previous `exposure`.

use crate::change_exposure::ChangeExposure;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeExposure, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_exposure::ChangeExposure> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeExposure, base: &En1996Snapshot) -> Option<ChangeExposure> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeExposure { id: payload.id.clone(), new_exposure: r0.exposure.clone() })
}
