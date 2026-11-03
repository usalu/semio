//! ↩️ `change-member-rd-str` inverse: restores the previous `rd_str`.

use crate::change_member_rd_str::ChangeMemberRdStr;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberRdStr, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_rd_str::ChangeMemberRdStr> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberRdStr, base: &En1990Snapshot) -> Option<ChangeMemberRdStr> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberRdStr { id: payload.id.clone(), new_rd_str: r0.rd_str })
}
