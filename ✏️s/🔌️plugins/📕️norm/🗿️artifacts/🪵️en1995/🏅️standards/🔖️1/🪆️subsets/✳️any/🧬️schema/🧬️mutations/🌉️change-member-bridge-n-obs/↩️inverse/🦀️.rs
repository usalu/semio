use super::ChangeMemberBridgeNObs;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberBridgeNObs, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };
    vec![En1995Mutation::ChangeMemberBridgeNObs(ChangeMemberBridgeNObs { member_id: payload.member_id.clone(), new_value: item.bridge_n_obs })]
}
