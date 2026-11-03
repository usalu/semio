//! ↩️ `change-foundation-footing-width-m` inverse: restores the previous `footing_width_m`.

use crate::change_foundation_footing_width_m::ChangeFoundationFootingWidthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationFootingWidthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_footing_width_m::ChangeFoundationFootingWidthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationFootingWidthM, base: &En1998Snapshot) -> Option<ChangeFoundationFootingWidthM> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationFootingWidthM { id: payload.id.clone(), new_footing_width_m: r0.footing_width_m })
}
