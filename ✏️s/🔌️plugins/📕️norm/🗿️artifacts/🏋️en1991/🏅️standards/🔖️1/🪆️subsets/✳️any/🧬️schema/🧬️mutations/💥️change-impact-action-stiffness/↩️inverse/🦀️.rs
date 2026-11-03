//! ↩️ `change-impact-action-stiffness` inverse: restores the previous `stiffness`.

use crate::change_impact_action_stiffness::ChangeImpactActionStiffness;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImpactActionStiffness, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_impact_action_stiffness::ChangeImpactActionStiffness> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImpactActionStiffness, base: &En1991Snapshot) -> Option<ChangeImpactActionStiffness> {
    let r0 = base.impact_actions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImpactActionStiffness { id: payload.id.clone(), new_stiffness: r0.stiffness })
}
