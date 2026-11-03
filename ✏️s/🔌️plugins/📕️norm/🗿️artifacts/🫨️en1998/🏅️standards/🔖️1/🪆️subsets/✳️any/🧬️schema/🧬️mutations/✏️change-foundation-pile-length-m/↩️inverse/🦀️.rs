//! ↩️ `change-foundation-pile-length-m` inverse: restores the previous `pile_length_m`.

use crate::change_foundation_pile_length_m::ChangeFoundationPileLengthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPileLengthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_pile_length_m::ChangeFoundationPileLengthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPileLengthM, base: &En1998Snapshot) -> Option<ChangeFoundationPileLengthM> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPileLengthM { id: payload.id.clone(), new_pile_length_m: r0.pile_length_m })
}
