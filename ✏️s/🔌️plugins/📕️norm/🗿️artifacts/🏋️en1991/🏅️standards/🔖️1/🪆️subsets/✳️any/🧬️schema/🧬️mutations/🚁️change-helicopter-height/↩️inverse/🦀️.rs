//! ↩️ `change-helicopter-height` inverse: restores the previous `height`.

use crate::change_helicopter_height::ChangeHelicopterHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeHelicopterHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_helicopter_height::ChangeHelicopterHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeHelicopterHeight, base: &En1991Snapshot) -> Option<ChangeHelicopterHeight> {
    let r0 = base.helicopter_landings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeHelicopterHeight { id: payload.id.clone(), new_height: r0.height })
}
