//! ↩️ Inverse for `change-member-detailing`.
use crate::change_member_detailing::ChangeMemberDetailing;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberDetailing, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_detailing::ChangeMemberDetailing> {
    Ok((|| {
    match base.buildings.get(payload.building_index).and_then(|b| b.members.get(payload.member_index)) {
        Some(m) => vec![Op::from(ChangeMemberDetailing { building_index: payload.building_index, member_index: payload.member_index, new_detailing_compatible_with_q: m.detailing_compatible_with_q })],
        None => Vec::new(),
    }

    })())
}
