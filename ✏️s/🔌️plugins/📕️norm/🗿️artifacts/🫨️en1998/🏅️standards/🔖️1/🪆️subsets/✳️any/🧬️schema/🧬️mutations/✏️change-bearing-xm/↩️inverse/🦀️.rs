//! ↩️ `change-bearing-xm` inverse: restores the previous `x_m`.

use crate::change_bearing_x_m::ChangeBearingXM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBearingXM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bearing_x_m::ChangeBearingXM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBearingXM, base: &En1998Snapshot) -> Option<ChangeBearingXM> {
    let r0 = base.bearings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBearingXM { id: payload.id.clone(), new_x_m: r0.x_m })
}
