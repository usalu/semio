//! ↩️ `change-member-buckling-length` inverse.

use crate::mutations::change_member_buckling_length::ChangeMemberBucklingLength;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(payload: &ChangeMemberBucklingLength, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    let len = base.members.iter().find(|m| m.id == payload.member_id).map(|m| match payload.axis.as_str() {
        "z" => m.buckling_length_z, "t" => m.buckling_length_t, "ltb" => m.ltb_length, _ => m.buckling_length_y
    }).unwrap_or(0.0);
    vec![En1999Mutation::ChangeMemberBucklingLength(ChangeMemberBucklingLength { member_id: payload.member_id.clone(), axis: payload.axis.clone(), new_length: len })]
}
