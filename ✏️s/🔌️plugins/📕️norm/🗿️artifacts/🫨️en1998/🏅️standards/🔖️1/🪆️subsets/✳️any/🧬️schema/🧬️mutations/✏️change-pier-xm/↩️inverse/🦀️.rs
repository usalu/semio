//! ↩️ `change-pier-xm` inverse: restores the previous `x_m`.

use crate::change_pier_x_m::ChangePierXM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierXM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_x_m::ChangePierXM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierXM, base: &En1998Snapshot) -> Option<ChangePierXM> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierXM { id: payload.id.clone(), new_x_m: r0.x_m })
}
