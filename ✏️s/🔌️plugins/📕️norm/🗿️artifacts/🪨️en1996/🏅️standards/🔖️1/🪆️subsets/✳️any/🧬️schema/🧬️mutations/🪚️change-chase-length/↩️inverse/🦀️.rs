//! ↩️ `change-chase-length` inverse: restores the previous `length_m`.

use crate::change_chase_length::ChangeChaseLength;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeChaseLength, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_chase_length::ChangeChaseLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeChaseLength, base: &En1996Snapshot) -> Option<ChangeChaseLength> {
    let r0 = base.chases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeChaseLength { id: payload.id.clone(), new_length_m: r0.length_m })
}
