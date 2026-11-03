//! ↩️ `change-member-stainless-forming` inverse: restores the previous `stainless_forming`.

use crate::change_member_stainless_forming::ChangeMemberStainlessForming;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberStainlessForming, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_stainless_forming::ChangeMemberStainlessForming> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberStainlessForming { id: payload.id.clone(), new_stainless_forming: row.stainless_forming })],
        None => Vec::new(),
    })
}
