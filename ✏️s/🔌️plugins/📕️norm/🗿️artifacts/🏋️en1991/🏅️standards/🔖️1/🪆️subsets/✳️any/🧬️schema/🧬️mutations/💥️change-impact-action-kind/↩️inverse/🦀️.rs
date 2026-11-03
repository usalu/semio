//! ↩️ `change-impact-action-kind` inverse: restores the previous `kind`.

use crate::change_impact_action_kind::ChangeImpactActionKind;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImpactActionKind, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_impact_action_kind::ChangeImpactActionKind> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImpactActionKind, base: &En1991Snapshot) -> Option<ChangeImpactActionKind> {
    let r0 = base.impact_actions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImpactActionKind { id: payload.id.clone(), new_kind: r0.kind })
}
