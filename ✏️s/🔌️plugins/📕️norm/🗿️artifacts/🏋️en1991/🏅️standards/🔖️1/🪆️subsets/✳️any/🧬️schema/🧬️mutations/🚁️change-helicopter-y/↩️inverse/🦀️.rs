//! ↩️ `change-helicopter-y` inverse: restores the previous `y`.

use crate::change_helicopter_y::ChangeHelicopterY;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeHelicopterY, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_helicopter_y::ChangeHelicopterY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeHelicopterY, base: &En1991Snapshot) -> Option<ChangeHelicopterY> {
    let r0 = base.helicopter_landings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeHelicopterY { id: payload.id.clone(), new_y: r0.y })
}
