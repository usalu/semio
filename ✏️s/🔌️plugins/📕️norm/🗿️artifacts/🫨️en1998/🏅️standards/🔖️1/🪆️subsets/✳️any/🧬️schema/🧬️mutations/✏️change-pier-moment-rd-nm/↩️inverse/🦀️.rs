//! ↩️ `change-pier-moment-rd-nm` inverse: restores the previous `moment_rd_nm`.

use crate::change_pier_moment_rd_nm::ChangePierMomentRdNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierMomentRdNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_moment_rd_nm::ChangePierMomentRdNm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierMomentRdNm, base: &En1998Snapshot) -> Option<ChangePierMomentRdNm> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierMomentRdNm { id: payload.id.clone(), new_moment_rd_nm: r0.moment_rd_nm })
}
