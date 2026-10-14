use crate::change_member_bridge_crowd::ChangeMemberBridgeCrowd;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeMemberBridgeCrowd, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_bridge_crowd::ChangeMemberBridgeCrowd> {
    Ok((|| {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };
    vec![Op::from(ChangeMemberBridgeCrowd { member_id: payload.member_id.clone(), new_value: item.bridge_crowd_per_m2 })]

    })())
}
