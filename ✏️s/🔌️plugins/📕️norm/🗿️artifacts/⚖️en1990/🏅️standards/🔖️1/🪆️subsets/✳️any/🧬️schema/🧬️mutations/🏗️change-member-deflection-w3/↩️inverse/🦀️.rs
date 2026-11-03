//! ↩️ `change-member-deflection-w3` inverse: restores the previous `deflection_w3`.

use crate::change_member_deflection_w3::ChangeMemberDeflectionW3;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberDeflectionW3, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_deflection_w3::ChangeMemberDeflectionW3> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberDeflectionW3, base: &En1990Snapshot) -> Option<ChangeMemberDeflectionW3> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberDeflectionW3 { id: payload.id.clone(), new_deflection_w3: r0.deflection_w3 })
}
