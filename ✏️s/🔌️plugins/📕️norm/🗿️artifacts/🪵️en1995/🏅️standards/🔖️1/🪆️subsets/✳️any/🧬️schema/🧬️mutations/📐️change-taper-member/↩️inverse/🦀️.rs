use crate::change_taper_member::ChangeTaperMember;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTaperMember, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_taper_member::ChangeTaperMember> {
    Ok(match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => vec![Op::from(ChangeTaperMember { taper_id: payload.taper_id.clone(), new_value: item.member_id.clone() })],
        None => Vec::new(),
    })
}
