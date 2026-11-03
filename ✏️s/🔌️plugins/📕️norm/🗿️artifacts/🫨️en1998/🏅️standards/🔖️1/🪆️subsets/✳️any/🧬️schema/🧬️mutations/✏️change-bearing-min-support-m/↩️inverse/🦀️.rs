//! ↩️ `change-bearing-min-support-m` inverse: restores the previous `min_support_length_m`.

use crate::change_bearing_min_support_m::ChangeBearingMinSupportM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBearingMinSupportM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bearing_min_support_m::ChangeBearingMinSupportM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBearingMinSupportM, base: &En1998Snapshot) -> Option<ChangeBearingMinSupportM> {
    let r0 = base.bearings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBearingMinSupportM { id: payload.id.clone(), new_min_support_length_m: r0.min_support_length_m })
}
