//! ↩️ `change-foundation-bearing-n-max-n` inverse: restores the previous `bearing_n_max_n`.

use crate::change_foundation_bearing_n_max_n::ChangeFoundationBearingNMaxN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationBearingNMaxN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_bearing_n_max_n::ChangeFoundationBearingNMaxN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationBearingNMaxN, base: &En1998Snapshot) -> Option<ChangeFoundationBearingNMaxN> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationBearingNMaxN { id: payload.id.clone(), new_bearing_n_max_n: r0.bearing_n_max_n })
}
