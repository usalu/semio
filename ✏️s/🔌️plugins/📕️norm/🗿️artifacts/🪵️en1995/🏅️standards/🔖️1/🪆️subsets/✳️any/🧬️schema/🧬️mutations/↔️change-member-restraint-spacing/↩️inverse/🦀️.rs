use crate::change_member_restraint_spacing::ChangeMemberRestraintSpacing;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeMemberRestraintSpacing, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_restraint_spacing::ChangeMemberRestraintSpacing> {
    Ok((|| {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };
    vec![Op::from(ChangeMemberRestraintSpacing { member_id: payload.member_id.clone(), new_value: item.lateral_restraint_spacing_m })]

    })())
}
