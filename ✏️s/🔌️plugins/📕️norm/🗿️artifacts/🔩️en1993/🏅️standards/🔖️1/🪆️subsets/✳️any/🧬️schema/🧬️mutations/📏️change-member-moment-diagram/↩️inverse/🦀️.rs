//! ↩️ `change-member-moment-diagram` inverse: restores the previous `moment_diagram`.

use crate::change_member_moment_diagram::ChangeMemberMomentDiagram;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMomentDiagram, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_moment_diagram::ChangeMemberMomentDiagram> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberMomentDiagram { id: payload.id.clone(), new_moment_diagram: row.moment_diagram })],
        None => Vec::new(),
    })
}
