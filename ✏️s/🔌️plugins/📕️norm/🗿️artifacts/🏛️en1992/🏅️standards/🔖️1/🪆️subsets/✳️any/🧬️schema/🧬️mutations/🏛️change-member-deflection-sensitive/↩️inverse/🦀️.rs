//! ↩️ `change-member-deflection-sensitive` inverse: restores the previous `deflection_sensitive`.

use crate::change_member_deflection_sensitive::ChangeMemberDeflectionSensitive;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberDeflectionSensitive, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_deflection_sensitive::ChangeMemberDeflectionSensitive> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberDeflectionSensitive, base: &En1992Snapshot) -> Option<ChangeMemberDeflectionSensitive> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    Some(ChangeMemberDeflectionSensitive { member_id: payload.member_id.clone(), new_deflection_sensitive: r0.deflection_sensitive })
}
