//! ↩️ `change-member-web-material-id` inverse: restores the previous `web_material_id`.

use crate::change_member_web_material_id::ChangeMemberWebMaterialId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberWebMaterialId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_web_material_id::ChangeMemberWebMaterialId> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberWebMaterialId { id: payload.id.clone(), new_web_material_id: row.web_material_id.clone() })],
        None => Vec::new(),
    })
}
