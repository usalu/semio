//! ↩️ `change-foundation-area-m2` inverse: restores the previous `area_m2`.

use crate::change_foundation_area_m2::ChangeFoundationAreaM2;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationAreaM2, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_area_m2::ChangeFoundationAreaM2> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationAreaM2, base: &En1998Snapshot) -> Option<ChangeFoundationAreaM2> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationAreaM2 { id: payload.id.clone(), new_area_m2: r0.area_m2 })
}
