//! ↩️ `change-derailment-assumed-force` inverse: restores the previous `assumed_force`.

use crate::change_derailment_assumed_force::ChangeDerailmentAssumedForce;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDerailmentAssumedForce, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_derailment_assumed_force::ChangeDerailmentAssumedForce> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDerailmentAssumedForce, base: &En1991Snapshot) -> Option<ChangeDerailmentAssumedForce> {
    let r0 = base.derailment_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDerailmentAssumedForce { id: payload.id.clone(), new_assumed_force: r0.assumed_force })
}
