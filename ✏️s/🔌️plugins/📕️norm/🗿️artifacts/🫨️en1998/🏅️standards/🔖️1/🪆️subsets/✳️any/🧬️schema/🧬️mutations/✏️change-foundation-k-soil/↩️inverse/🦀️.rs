//! ↩️ `change-foundation-k-soil` inverse: restores the previous `k_soil`.

use crate::change_foundation_k_soil::ChangeFoundationKSoil;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationKSoil, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_k_soil::ChangeFoundationKSoil> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationKSoil, base: &En1998Snapshot) -> Option<ChangeFoundationKSoil> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationKSoil { id: payload.id.clone(), new_k_soil: r0.k_soil })
}
