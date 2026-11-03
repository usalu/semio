//! ↩️ `change-member-label-en` inverse: restores the previous `label_en`.

use crate::change_member_label_en::ChangeMemberLabelEn;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberLabelEn, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_label_en::ChangeMemberLabelEn> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberLabelEn, base: &En1990Snapshot) -> Option<ChangeMemberLabelEn> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberLabelEn { id: payload.id.clone(), new_label_en: r0.label_en.clone() })
}
