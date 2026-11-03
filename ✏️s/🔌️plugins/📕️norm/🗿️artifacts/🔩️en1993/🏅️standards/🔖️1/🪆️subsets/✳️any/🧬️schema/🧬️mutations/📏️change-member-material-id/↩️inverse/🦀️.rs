//! ↩️ `change-member-material-id` inverse: restores the previous `material_id`.

use crate::change_member_material_id::ChangeMemberMaterialId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMaterialId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_material_id::ChangeMemberMaterialId> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberMaterialId { id: payload.id.clone(), new_material_id: row.material_id.clone() })],
        None => Vec::new(),
    })
}
