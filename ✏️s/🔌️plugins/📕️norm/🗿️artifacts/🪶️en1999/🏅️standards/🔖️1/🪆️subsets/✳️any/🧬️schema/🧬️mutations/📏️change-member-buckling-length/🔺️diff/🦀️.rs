//! 🔺️ `change-member-buckling-length` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_member_buckling_length::ChangeMemberBucklingLength;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeMemberBucklingLength, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_length.is_finite() || payload.new_length <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Length must be positive.", Vec::<String>::new());
    }
    let mut members = base.members.clone();
    let Some(member) = members.iter_mut().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown member {}", payload.member_id), Vec::<String>::new());
    };
    match payload.axis.as_str() {
        "z" => member.buckling_length_z = payload.new_length,
        "t" => member.buckling_length_t = payload.new_length,
        "ltb" => member.ltb_length = payload.new_length,
        _ => member.buckling_length_y = payload.new_length,
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(members), ..Default::default() })
}
