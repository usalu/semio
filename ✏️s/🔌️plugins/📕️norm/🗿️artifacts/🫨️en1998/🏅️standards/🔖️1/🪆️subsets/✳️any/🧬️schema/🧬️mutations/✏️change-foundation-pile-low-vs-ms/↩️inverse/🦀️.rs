//! ↩️ `change-foundation-pile-low-vs-ms` inverse: restores the previous `pile_lower_vs_m_s`.

use crate::change_foundation_pile_low_vs_m_s::ChangeFoundationPileLowVsMS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPileLowVsMS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_pile_low_vs_m_s::ChangeFoundationPileLowVsMS> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPileLowVsMS, base: &En1998Snapshot) -> Option<ChangeFoundationPileLowVsMS> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPileLowVsMS { id: payload.id.clone(), new_pile_lower_vs_m_s: r0.pile_lower_vs_m_s })
}
