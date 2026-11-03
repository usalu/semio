//! ↩️ `change-imperfection-assumed-force` inverse: restores the previous `assumed_force`.

use crate::change_imperfection_assumed_force::ChangeImperfectionAssumedForce;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImperfectionAssumedForce, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_imperfection_assumed_force::ChangeImperfectionAssumedForce> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImperfectionAssumedForce, base: &En1991Snapshot) -> Option<ChangeImperfectionAssumedForce> {
    let r0 = base.execution_imperfections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImperfectionAssumedForce { id: payload.id.clone(), new_assumed_force: r0.assumed_force })
}
