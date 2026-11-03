//! ↩️ `change-patch-subject-id` inverse: restores the previous `subject_id`.

use crate::change_patch_subject_id::ChangePatchSubjectId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePatchSubjectId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_patch_subject_id::ChangePatchSubjectId> {
    Ok(match base.patch_loads.iter().find(|row| row.id == payload.patch_load_id) {
        Some(row) => vec![Op::from(ChangePatchSubjectId { patch_load_id: payload.patch_load_id.clone(), new_subject_id: row.subject_id.clone() })],
        None => Vec::new(),
    })
}
