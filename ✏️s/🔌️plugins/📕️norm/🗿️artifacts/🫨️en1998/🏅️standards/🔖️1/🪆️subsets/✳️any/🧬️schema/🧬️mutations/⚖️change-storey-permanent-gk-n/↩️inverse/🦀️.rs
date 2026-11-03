//! ↩️ `change-storey-permanent-gk-n` inverse: restores the previous `permanent_gk_n`.

use crate::change_storey_permanent_gk_n::ChangeStoreyPermanentGkN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyPermanentGkN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_permanent_gk_n::ChangeStoreyPermanentGkN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStoreyPermanentGkN, base: &En1998Snapshot) -> Option<ChangeStoreyPermanentGkN> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.storeys.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStoreyPermanentGkN { building_id: payload.building_id.clone(), id: payload.id.clone(), new_permanent_gk_n: r1.permanent_gk_n })
}
