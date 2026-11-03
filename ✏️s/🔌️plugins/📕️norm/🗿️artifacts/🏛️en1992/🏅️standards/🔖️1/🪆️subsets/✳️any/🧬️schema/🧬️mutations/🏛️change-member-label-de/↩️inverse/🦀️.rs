//! ↩️ `change-member-label-de` inverse: restores the previous `label_de`.

use crate::change_member_label_de::ChangeMemberLabelDe;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberLabelDe, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_label_de::ChangeMemberLabelDe> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberLabelDe, base: &En1992Snapshot) -> Option<ChangeMemberLabelDe> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    Some(ChangeMemberLabelDe { member_id: payload.member_id.clone(), new_label_de: r0.label_de.clone() })
}
