//! ↩️ `change-bridge-bearing-d-rd-m` inverse: restores the previous `bearing_d_rd_m`.

use crate::change_bridge_bearing_d_rd_m::ChangeBridgeBearingDRdM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeBearingDRdM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_bearing_d_rd_m::ChangeBridgeBearingDRdM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeBearingDRdM, base: &En1998Snapshot) -> Option<ChangeBridgeBearingDRdM> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeBearingDRdM { id: payload.id.clone(), new_bearing_d_rd_m: r0.bearing_d_rd_m })
}
