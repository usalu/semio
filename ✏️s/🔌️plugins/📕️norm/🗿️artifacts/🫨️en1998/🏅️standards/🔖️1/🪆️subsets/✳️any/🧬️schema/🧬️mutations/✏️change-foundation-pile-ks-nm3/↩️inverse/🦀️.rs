//! ↩️ `change-foundation-pile-ks-nm3` inverse: restores the previous `pile_subgrade_modulus_n_m3`.

use crate::change_foundation_pile_ks_n_m3::ChangeFoundationPileKsNM3;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPileKsNM3, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_pile_ks_n_m3::ChangeFoundationPileKsNM3> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPileKsNM3, base: &En1998Snapshot) -> Option<ChangeFoundationPileKsNM3> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPileKsNM3 { id: payload.id.clone(), new_pile_subgrade_modulus_n_m3: r0.pile_subgrade_modulus_n_m3 })
}
