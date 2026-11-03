//! ↩️ `change-foundation-k-foundation` inverse: restores the previous `k_foundation`.

use crate::change_foundation_k_foundation::ChangeFoundationKFoundation;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationKFoundation, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_k_foundation::ChangeFoundationKFoundation> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationKFoundation, base: &En1998Snapshot) -> Option<ChangeFoundationKFoundation> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationKFoundation { id: payload.id.clone(), new_k_foundation: r0.k_foundation })
}
