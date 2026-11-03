//! ↩️ `change-category-member-id` inverse: restores the previous `member_id`.

use crate::change_category_member_id::ChangeCategoryMemberId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeCategoryMemberId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_category_member_id::ChangeCategoryMemberId> {
    Ok(match base.fatigue_categories.iter().find(|row| row.id == payload.fatigue_category_id) {
        Some(row) => vec![Op::from(ChangeCategoryMemberId { fatigue_category_id: payload.fatigue_category_id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
