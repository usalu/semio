//! ↩️ `change-self-weight-assumed-gk` inverse: restores the previous `assumed_gk`.

use crate::change_self_weight_assumed_gk::ChangeSelfWeightAssumedGk;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSelfWeightAssumedGk, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_self_weight_assumed_gk::ChangeSelfWeightAssumedGk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSelfWeightAssumedGk, base: &En1991Snapshot) -> Option<ChangeSelfWeightAssumedGk> {
    let r0 = base.self_weight_elements.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSelfWeightAssumedGk { id: payload.id.clone(), new_assumed_gk: r0.assumed_gk })
}
