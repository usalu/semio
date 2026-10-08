//! 🔺️ `change-member-buckling-length` diff.

use crate::mutations::change_member_buckling_length::ChangeMemberBucklingLength;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999MembersRows, En1999MembersPatch};

pub fn diff(payload: &ChangeMemberBucklingLength, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_length.is_finite() || payload.new_length <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Length must be positive.", Vec::<String>::new());
    }
    if !base.members.iter().any(|member| member.id == payload.member_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown member {}", payload.member_id), Vec::<String>::new());
    }
    let id = payload.member_id.clone();
    let value = Some(payload.new_length);
    let patch = match payload.axis.as_str() {
        "z" => En1999MembersPatch { id, buckling_length_z: value, ..Default::default() },
        "t" => En1999MembersPatch { id, buckling_length_t: value, ..Default::default() },
        "ltb" => En1999MembersPatch { id, ltb_length: value, ..Default::default() },
        _ => En1999MembersPatch { id, buckling_length_y: value, ..Default::default() },
    };
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows { modified: vec![patch], ..Default::default() }), ..Default::default() })
}
