//! Diff for `change-member-detailing-compatible`.
use super::ChangeMemberDetailingCompatible;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeMemberDetailingCompatible, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut buildings = base.buildings.clone();
    let Some(b) = buildings.get_mut(payload.building_index) else { return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new()); };
    let Some(m) = b.members.get_mut(payload.member_index) else { return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new()); };
    m.detailing_compatible_with_q = payload.new_detailing_compatible_with_q;
    protocol::MutationOutcome::new(En1998Diff { buildings: Some(buildings), ..Default::default() })
}
