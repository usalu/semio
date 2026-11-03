//! ↩️ `change-accelerogram-dt-s` inverse: restores the previous `dt_s`.

use crate::change_accelerogram_dt_s::ChangeAccelerogramDtS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAccelerogramDtS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accelerogram_dt_s::ChangeAccelerogramDtS> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccelerogramDtS, base: &En1998Snapshot) -> Option<ChangeAccelerogramDtS> {
    let r0 = base.accelerograms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAccelerogramDtS { id: payload.id.clone(), new_dt_s: r0.dt_s })
}
