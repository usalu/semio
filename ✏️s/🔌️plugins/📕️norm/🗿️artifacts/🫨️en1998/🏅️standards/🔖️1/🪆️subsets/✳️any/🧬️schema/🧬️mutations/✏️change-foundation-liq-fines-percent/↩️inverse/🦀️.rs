//! ↩️ `change-foundation-liq-fines-percent` inverse: restores the previous `liq_fines_percent`.

use crate::change_foundation_liq_fines_percent::ChangeFoundationLiqFinesPercent;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationLiqFinesPercent, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_liq_fines_percent::ChangeFoundationLiqFinesPercent> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationLiqFinesPercent, base: &En1998Snapshot) -> Option<ChangeFoundationLiqFinesPercent> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationLiqFinesPercent { id: payload.id.clone(), new_liq_fines_percent: r0.liq_fines_percent })
}
