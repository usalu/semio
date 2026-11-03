//! ↩️ `change-foundation-pile-up-vs-ms` inverse: restores the previous `pile_upper_vs_m_s`.

use crate::change_foundation_pile_up_vs_m_s::ChangeFoundationPileUpVsMS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPileUpVsMS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_pile_up_vs_m_s::ChangeFoundationPileUpVsMS> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPileUpVsMS, base: &En1998Snapshot) -> Option<ChangeFoundationPileUpVsMS> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPileUpVsMS { id: payload.id.clone(), new_pile_upper_vs_m_s: r0.pile_upper_vs_m_s })
}
