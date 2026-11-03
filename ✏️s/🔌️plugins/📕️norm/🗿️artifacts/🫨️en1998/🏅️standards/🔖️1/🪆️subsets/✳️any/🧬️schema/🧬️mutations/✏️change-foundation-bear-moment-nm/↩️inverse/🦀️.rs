//! ↩️ `change-foundation-bear-moment-nm` inverse: restores the previous `bearing_moment_nm`.

use crate::change_foundation_bear_moment_nm::ChangeFoundationBearMomentNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationBearMomentNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_bear_moment_nm::ChangeFoundationBearMomentNm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationBearMomentNm, base: &En1998Snapshot) -> Option<ChangeFoundationBearMomentNm> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationBearMomentNm { id: payload.id.clone(), new_bearing_moment_nm: r0.bearing_moment_nm })
}
