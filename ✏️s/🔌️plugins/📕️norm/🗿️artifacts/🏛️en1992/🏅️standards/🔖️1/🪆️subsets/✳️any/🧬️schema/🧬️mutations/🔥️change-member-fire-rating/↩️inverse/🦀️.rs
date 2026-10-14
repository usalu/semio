use crate::change_member_fire_rating::ChangeMemberFireRating;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberFireRating, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_fire_rating::ChangeMemberFireRating> {
    Ok((|| {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    let Some(f) = m.fire.as_ref() else { return vec![]; };
    vec![Op::from(ChangeMemberFireRating { member_id: payload.member_id.clone(), new_rating: f.rating })]

    })())
}
