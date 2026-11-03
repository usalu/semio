//! ↩️ `change-member-deflection-w2` inverse: restores the previous `deflection_w2`.

use crate::change_member_deflection_w2::ChangeMemberDeflectionW2;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberDeflectionW2, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_deflection_w2::ChangeMemberDeflectionW2> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberDeflectionW2, base: &En1990Snapshot) -> Option<ChangeMemberDeflectionW2> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberDeflectionW2 { id: payload.id.clone(), new_deflection_w2: r0.deflection_w2 })
}
