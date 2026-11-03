//! ↩️ `change-derailment-distance` inverse: restores the previous `distance`.

use crate::change_derailment_distance::ChangeDerailmentDistance;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDerailmentDistance, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_derailment_distance::ChangeDerailmentDistance> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDerailmentDistance, base: &En1991Snapshot) -> Option<ChangeDerailmentDistance> {
    let r0 = base.derailment_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDerailmentDistance { id: payload.id.clone(), new_distance: r0.distance })
}
