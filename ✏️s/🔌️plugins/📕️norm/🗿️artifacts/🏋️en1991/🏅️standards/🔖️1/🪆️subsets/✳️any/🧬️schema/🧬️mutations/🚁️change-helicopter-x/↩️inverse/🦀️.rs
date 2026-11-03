//! ↩️ `change-helicopter-x` inverse: restores the previous `x`.

use crate::change_helicopter_x::ChangeHelicopterX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeHelicopterX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_helicopter_x::ChangeHelicopterX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeHelicopterX, base: &En1991Snapshot) -> Option<ChangeHelicopterX> {
    let r0 = base.helicopter_landings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeHelicopterX { id: payload.id.clone(), new_x: r0.x })
}
