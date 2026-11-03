//! ↩️ `change-bridge-pier-moment-rd-nm` inverse: restores the previous `pier_moment_rd_nm`.

use crate::change_bridge_pier_moment_rd_nm::ChangeBridgePierMomentRdNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgePierMomentRdNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_pier_moment_rd_nm::ChangeBridgePierMomentRdNm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgePierMomentRdNm, base: &En1998Snapshot) -> Option<ChangeBridgePierMomentRdNm> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgePierMomentRdNm { id: payload.id.clone(), new_pier_moment_rd_nm: r0.pier_moment_rd_nm })
}
