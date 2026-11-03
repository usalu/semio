//! ↩️ `change-pier-cover-m` inverse: restores the previous `cover_m`.

use crate::change_pier_cover_m::ChangePierCoverM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierCoverM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_cover_m::ChangePierCoverM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierCoverM, base: &En1998Snapshot) -> Option<ChangePierCoverM> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierCoverM { id: payload.id.clone(), new_cover_m: r0.cover_m })
}
