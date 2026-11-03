//! ↩️ `change-foundation-liq-n160` inverse: restores the previous `liq_n1_60`.

use crate::change_foundation_liq_n160::ChangeFoundationLiqN160;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationLiqN160, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_liq_n160::ChangeFoundationLiqN160> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationLiqN160, base: &En1998Snapshot) -> Option<ChangeFoundationLiqN160> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationLiqN160 { id: payload.id.clone(), new_liq_n160: r0.liq_n1_60 })
}
