//! Inverse for `change-member-detailing-compatible`.
use super::ChangeMemberDetailingCompatible;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeMemberDetailingCompatible, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.buildings.get(payload.building_index).and_then(|b| b.members.get(payload.member_index)) {
        Some(m) => vec![En1998Mutation::ChangeMemberDetailingCompatible(ChangeMemberDetailingCompatible { building_index: payload.building_index, member_index: payload.member_index, new_detailing_compatible_with_q: m.detailing_compatible_with_q })],
        None => Vec::new(),
    }
}
