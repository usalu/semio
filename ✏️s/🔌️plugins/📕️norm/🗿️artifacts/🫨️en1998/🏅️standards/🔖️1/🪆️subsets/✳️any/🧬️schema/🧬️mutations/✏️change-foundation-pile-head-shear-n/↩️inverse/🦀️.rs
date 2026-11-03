//! ↩️ `change-foundation-pile-head-shear-n` inverse: restores the previous `pile_head_shear_n`.

use crate::change_foundation_pile_head_shear_n::ChangeFoundationPileHeadShearN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPileHeadShearN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_pile_head_shear_n::ChangeFoundationPileHeadShearN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPileHeadShearN, base: &En1998Snapshot) -> Option<ChangeFoundationPileHeadShearN> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPileHeadShearN { id: payload.id.clone(), new_pile_head_shear_n: r0.pile_head_shear_n })
}
