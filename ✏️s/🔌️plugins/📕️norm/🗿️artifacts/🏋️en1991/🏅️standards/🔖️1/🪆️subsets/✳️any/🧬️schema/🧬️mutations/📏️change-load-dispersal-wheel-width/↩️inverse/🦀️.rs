//! ↩️ `change-load-dispersal-wheel-width` inverse: restores the previous `wheel_width`.

use crate::change_load_dispersal_wheel_width::ChangeLoadDispersalWheelWidth;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadDispersalWheelWidth, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_dispersal_wheel_width::ChangeLoadDispersalWheelWidth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLoadDispersalWheelWidth, base: &En1991Snapshot) -> Option<ChangeLoadDispersalWheelWidth> {
    let r0 = base.load_dispersals.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLoadDispersalWheelWidth { id: payload.id.clone(), new_wheel_width: r0.wheel_width })
}
