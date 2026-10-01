use super::ChangeMemberMassKgPerM2;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberMassKgPerM2, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };
    vec![En1995Mutation::ChangeMemberMassKgPerM2(ChangeMemberMassKgPerM2 { member_id: payload.member_id.clone(), new_value: item.mass_kg_per_m2 })]
}
