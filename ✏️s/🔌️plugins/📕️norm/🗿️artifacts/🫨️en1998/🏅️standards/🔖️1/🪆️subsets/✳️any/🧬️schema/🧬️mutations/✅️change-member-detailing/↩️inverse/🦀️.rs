//! Inverse for `change-member-detailing`.
use super::ChangeMemberDetailing;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeMemberDetailing, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.buildings.get(payload.building_index).and_then(|b| b.members.get(payload.member_index)) {
        Some(m) => vec![En1998Mutation::ChangeMemberDetailing(ChangeMemberDetailing { building_index: payload.building_index, member_index: payload.member_index, new_detailing_compatible_with_q: m.detailing_compatible_with_q })],
        None => Vec::new(),
    }

    })())
}
