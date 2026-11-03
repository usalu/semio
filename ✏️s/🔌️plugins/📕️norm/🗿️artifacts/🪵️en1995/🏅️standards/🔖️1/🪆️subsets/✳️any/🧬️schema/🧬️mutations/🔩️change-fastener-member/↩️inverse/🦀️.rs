use crate::change_fastener_member::ChangeFastenerMember;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFastenerMember, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastener_member::ChangeFastenerMember> {
    Ok(match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => vec![Op::from(ChangeFastenerMember { fastener_id: payload.fastener_id.clone(), new_value: item.member_id.clone() })],
        None => Vec::new(),
    })
}
