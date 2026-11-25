use crate::change_member_mass_kg_per_m::ChangeMemberMassKgPerM;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberMassKgPerM, base: &En1995Snapshot) -> Option<crate::change_member_mass_kg_per_m::ChangeMemberMassKgPerM> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    Some(ChangeMemberMassKgPerM { member_id: payload.member_id.clone(), new_value: item.mass_kg_per_m })
}
