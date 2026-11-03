//! ↩️ `change-foundation-pile-moment-rd-nm` inverse: restores the previous `pile_moment_rd_nm`.

use crate::change_foundation_pile_moment_rd_nm::ChangeFoundationPileMomentRdNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPileMomentRdNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_pile_moment_rd_nm::ChangeFoundationPileMomentRdNm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPileMomentRdNm, base: &En1998Snapshot) -> Option<ChangeFoundationPileMomentRdNm> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPileMomentRdNm { id: payload.id.clone(), new_pile_moment_rd_nm: r0.pile_moment_rd_nm })
}
