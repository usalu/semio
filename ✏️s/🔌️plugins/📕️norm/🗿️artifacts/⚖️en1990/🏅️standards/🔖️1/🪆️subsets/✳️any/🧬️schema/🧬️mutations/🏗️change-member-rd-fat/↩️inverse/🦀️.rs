//! ↩️ `change-member-rd-fat` inverse: restores the previous `rd_fat`.

use crate::change_member_rd_fat::ChangeMemberRdFat;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberRdFat, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_rd_fat::ChangeMemberRdFat> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberRdFat, base: &En1990Snapshot) -> Option<ChangeMemberRdFat> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberRdFat { id: payload.id.clone(), new_rd_fat: r0.rd_fat })
}
