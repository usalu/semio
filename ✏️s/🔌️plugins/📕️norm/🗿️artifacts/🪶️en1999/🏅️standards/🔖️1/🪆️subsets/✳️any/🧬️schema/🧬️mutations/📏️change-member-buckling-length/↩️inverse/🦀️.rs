//! ↩️ `change-member-buckling-length` inverse.

use crate::change_member_buckling_length::ChangeMemberBucklingLength;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberBucklingLength, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_buckling_length::ChangeMemberBucklingLength> {
    Ok((|| {
    let len = base.members.iter().find(|m| m.id == payload.member_id).map(|m| match payload.axis.as_str() {
        "z" => m.buckling_length_z, "t" => m.buckling_length_t, "ltb" => m.ltb_length, _ => m.buckling_length_y
    }).unwrap_or(0.0);
    vec![Op::from(ChangeMemberBucklingLength { member_id: payload.member_id.clone(), axis: payload.axis.clone(), new_length: len })]

    })())
}
