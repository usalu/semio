//! ↩️ `change-foundation-pile-diameter-m` inverse: restores the previous `pile_diameter_m`.

use crate::change_foundation_pile_diameter_m::ChangeFoundationPileDiameterM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPileDiameterM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_pile_diameter_m::ChangeFoundationPileDiameterM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPileDiameterM, base: &En1998Snapshot) -> Option<ChangeFoundationPileDiameterM> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPileDiameterM { id: payload.id.clone(), new_pile_diameter_m: r0.pile_diameter_m })
}
