//! ↩️ `change-member-load-application` inverse: restores the previous `load_application`.

use crate::change_member_load_application::ChangeMemberLoadApplication;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberLoadApplication, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_load_application::ChangeMemberLoadApplication> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberLoadApplication { id: payload.id.clone(), new_load_application: row.load_application })],
        None => Vec::new(),
    })
}
