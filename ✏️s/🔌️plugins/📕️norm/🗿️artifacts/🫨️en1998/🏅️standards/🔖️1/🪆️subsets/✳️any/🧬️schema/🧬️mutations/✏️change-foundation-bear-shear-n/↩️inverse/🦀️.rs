//! ↩️ `change-foundation-bear-shear-n` inverse: restores the previous `bearing_shear_n`.

use crate::change_foundation_bear_shear_n::ChangeFoundationBearShearN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationBearShearN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_bear_shear_n::ChangeFoundationBearShearN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationBearShearN, base: &En1998Snapshot) -> Option<ChangeFoundationBearShearN> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationBearShearN { id: payload.id.clone(), new_bearing_shear_n: r0.bearing_shear_n })
}
