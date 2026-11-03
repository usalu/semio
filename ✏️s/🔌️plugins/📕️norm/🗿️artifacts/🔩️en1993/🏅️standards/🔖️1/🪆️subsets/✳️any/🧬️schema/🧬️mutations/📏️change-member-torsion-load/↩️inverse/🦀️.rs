//! ↩️ `change-member-torsion-load` inverse: restores the previous `torsion_load`.

use crate::change_member_torsion_load::ChangeMemberTorsionLoad;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberTorsionLoad, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_torsion_load::ChangeMemberTorsionLoad> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberTorsionLoad { id: payload.id.clone(), new_torsion_load: row.torsion_load })],
        None => Vec::new(),
    })
}
