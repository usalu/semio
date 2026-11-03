use crate::change_cutout_member::ChangeCutoutMember;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeCutoutMember, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cutout_member::ChangeCutoutMember> {
    Ok(match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => vec![Op::from(ChangeCutoutMember { cutout_id: payload.cutout_id.clone(), new_value: item.member_id.clone() })],
        None => Vec::new(),
    })
}
