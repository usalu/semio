//! ↩️ `change-frame-member-basis-basis` inverse: restores the previous `basis`.

use crate::change_frame_member_basis_basis::ChangeFrameMemberBasisBasis;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameMemberBasisBasis, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_member_basis_basis::ChangeFrameMemberBasisBasis> {
    Ok(match base.frame_member_bases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameMemberBasisBasis { id: payload.id.clone(), new_basis: row.basis })],
        None => Vec::new(),
    })
}
