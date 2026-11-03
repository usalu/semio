//! ↩️ `change-member-material-id` inverse: restores the previous `material_id`.

use crate::change_member_material_id::ChangeMemberMaterialId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMaterialId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_material_id::ChangeMemberMaterialId> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberMaterialId { member_id: payload.member_id.clone(), new_material_id: row.material_id.clone() })],
        None => Vec::new(),
    })
}
