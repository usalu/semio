//! ↩️ `change-foundation-liq-magnitude` inverse: restores the previous `liq_magnitude`.

use crate::change_foundation_liq_magnitude::ChangeFoundationLiqMagnitude;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationLiqMagnitude, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_liq_magnitude::ChangeFoundationLiqMagnitude> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationLiqMagnitude, base: &En1998Snapshot) -> Option<ChangeFoundationLiqMagnitude> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationLiqMagnitude { id: payload.id.clone(), new_liq_magnitude: r0.liq_magnitude })
}
