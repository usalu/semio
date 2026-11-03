use crate::change_member_order::ChangeMemberOrder;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberOrder, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_order::ChangeMemberOrder> {
    Ok((|| {
    if payload.from_index >= base.members.len() {
        return Vec::new();
    }
    let moved_to = payload.to_index.min(base.members.len() - 1);
    vec![Op::from(ChangeMemberOrder { from_index: moved_to, to_index: payload.from_index })]

    })())
}
