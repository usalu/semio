//! 🏛️ `change-member-fire-slab-system` diff: patches `fire.slab_system` of the addressed member row.

use crate::change_member_fire_slab_system::ChangeMemberFireSlabSystem;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeMemberFireSlabSystem, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(object) = r0.fire.as_ref() else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The optional fire is absent.", Vec::<String>::new());
    };
    if object.slab_system == payload.new_fire_slab_system {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { fire_slab_system: Some(payload.new_fire_slab_system), ..Default::default() })), ..Default::default() })
}
