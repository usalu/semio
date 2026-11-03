//! ↩️ `change-member-sls-limit-char` inverse: restores the previous `sls_limit_char`.

use crate::change_member_sls_limit_char::ChangeMemberSlsLimitChar;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberSlsLimitChar, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_sls_limit_char::ChangeMemberSlsLimitChar> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberSlsLimitChar, base: &En1990Snapshot) -> Option<ChangeMemberSlsLimitChar> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberSlsLimitChar { id: payload.id.clone(), new_sls_limit_char: r0.sls_limit_char })
}
