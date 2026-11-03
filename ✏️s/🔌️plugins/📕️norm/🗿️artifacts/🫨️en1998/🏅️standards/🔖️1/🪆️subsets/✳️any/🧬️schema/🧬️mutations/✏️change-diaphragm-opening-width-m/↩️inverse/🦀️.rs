//! ↩️ `change-diaphragm-opening-width-m` inverse: restores the previous `opening_width_m`.

use crate::change_diaphragm_opening_width_m::ChangeDiaphragmOpeningWidthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeDiaphragmOpeningWidthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_opening_width_m::ChangeDiaphragmOpeningWidthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDiaphragmOpeningWidthM, base: &En1998Snapshot) -> Option<ChangeDiaphragmOpeningWidthM> {
    let r0 = base.diaphragms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDiaphragmOpeningWidthM { id: payload.id.clone(), new_opening_width_m: r0.opening_width_m })
}
