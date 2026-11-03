use crate::change_member_tightness::ChangeMemberTightness;
use crate::clear_member_tightness::ClearMemberTightness;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberTightness, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_tightness::ChangeMemberTightness> + From<crate::clear_member_tightness::ClearMemberTightness> {
    Ok((|| {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    vec![match m.tightness {
        Some(previous) => Op::from(ChangeMemberTightness { member_id: payload.member_id.clone(), new_tightness: previous }),
        None => Op::from(ClearMemberTightness { member_id: payload.member_id.clone() }),
    }]
    })())
}
