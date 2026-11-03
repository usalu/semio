//! ↩️ `change-foundation-pile-head-fixed` inverse: restores the previous `pile_head_fixed`.

use crate::change_foundation_pile_head_fixed::ChangeFoundationPileHeadFixed;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPileHeadFixed, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_pile_head_fixed::ChangeFoundationPileHeadFixed> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPileHeadFixed, base: &En1998Snapshot) -> Option<ChangeFoundationPileHeadFixed> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPileHeadFixed { id: payload.id.clone(), new_pile_head_fixed: r0.pile_head_fixed })
}
