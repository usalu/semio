use crate::change_member_axis_distance::ChangeMemberAxisDistance;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberAxisDistance, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_axis_distance::ChangeMemberAxisDistance> {
    Ok((|| {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    let Some(f) = m.fire.as_ref() else { return vec![]; };
    vec![Op::from(ChangeMemberAxisDistance { member_id: payload.member_id.clone(), new_axis_distance: f.axis_distance })]

    })())
}
