//! ↩️ `change-impact-action-mass` inverse: restores the previous `mass`.

use crate::change_impact_action_mass::ChangeImpactActionMass;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImpactActionMass, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_impact_action_mass::ChangeImpactActionMass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImpactActionMass, base: &En1991Snapshot) -> Option<ChangeImpactActionMass> {
    let r0 = base.impact_actions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImpactActionMass { id: payload.id.clone(), new_mass: r0.mass })
}
