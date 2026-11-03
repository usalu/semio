//! ↩️ `change-bearing-stiffness-n-per-m` inverse: restores the previous `stiffness_n_per_m`.

use crate::change_bearing_stiffness_n_per_m::ChangeBearingStiffnessNPerM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBearingStiffnessNPerM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bearing_stiffness_n_per_m::ChangeBearingStiffnessNPerM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBearingStiffnessNPerM, base: &En1998Snapshot) -> Option<ChangeBearingStiffnessNPerM> {
    let r0 = base.bearings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBearingStiffnessNPerM { id: payload.id.clone(), new_stiffness_n_per_m: r0.stiffness_n_per_m })
}
