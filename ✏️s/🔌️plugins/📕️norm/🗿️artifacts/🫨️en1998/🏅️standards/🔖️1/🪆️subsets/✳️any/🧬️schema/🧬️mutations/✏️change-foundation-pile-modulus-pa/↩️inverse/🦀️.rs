//! ↩️ `change-foundation-pile-modulus-pa` inverse: restores the previous `pile_modulus_pa`.

use crate::change_foundation_pile_modulus_pa::ChangeFoundationPileModulusPa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPileModulusPa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_pile_modulus_pa::ChangeFoundationPileModulusPa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPileModulusPa, base: &En1998Snapshot) -> Option<ChangeFoundationPileModulusPa> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPileModulusPa { id: payload.id.clone(), new_pile_modulus_pa: r0.pile_modulus_pa })
}
