//! ↩️ `change-diaphragm-thickness-m` inverse: restores the previous `thickness_m`.

use crate::change_diaphragm_thickness_m::ChangeDiaphragmThicknessM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeDiaphragmThicknessM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_thickness_m::ChangeDiaphragmThicknessM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDiaphragmThicknessM, base: &En1998Snapshot) -> Option<ChangeDiaphragmThicknessM> {
    let r0 = base.diaphragms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDiaphragmThicknessM { id: payload.id.clone(), new_thickness_m: r0.thickness_m })
}
