use crate::change_member_restraint_spacing::ChangeMemberRestraintSpacing;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberRestraintSpacing, base: &En1995Snapshot) -> Option<crate::change_member_restraint_spacing::ChangeMemberRestraintSpacing> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberRestraintSpacing { member_id: payload.member_id.clone(), new_value: item.lateral_restraint_spacing_m })
}
