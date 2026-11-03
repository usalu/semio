//! ↩️ `change-bearing-d-rd-m` inverse: restores the previous `d_rd_m`.

use crate::change_bearing_d_rd_m::ChangeBearingDRdM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBearingDRdM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bearing_d_rd_m::ChangeBearingDRdM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBearingDRdM, base: &En1998Snapshot) -> Option<ChangeBearingDRdM> {
    let r0 = base.bearings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBearingDRdM { id: payload.id.clone(), new_d_rd_m: r0.d_rd_m })
}
