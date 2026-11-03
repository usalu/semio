//! ↩️ `change-foundation-bear-axial-n` inverse: restores the previous `bearing_axial_n`.

use crate::change_foundation_bear_axial_n::ChangeFoundationBearAxialN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationBearAxialN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_bear_axial_n::ChangeFoundationBearAxialN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationBearAxialN, base: &En1998Snapshot) -> Option<ChangeFoundationBearAxialN> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationBearAxialN { id: payload.id.clone(), new_bearing_axial_n: r0.bearing_axial_n })
}
