//! ↩️ `change-local-fire-heat-release-rate` inverse: restores the previous `heat_release_rate`.

use crate::change_local_fire_heat_release_rate::ChangeLocalFireHeatReleaseRate;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLocalFireHeatReleaseRate, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_local_fire_heat_release_rate::ChangeLocalFireHeatReleaseRate> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLocalFireHeatReleaseRate, base: &En1991Snapshot) -> Option<ChangeLocalFireHeatReleaseRate> {
    let r0 = base.local_fires.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLocalFireHeatReleaseRate { id: payload.id.clone(), new_heat_release_rate: r0.heat_release_rate })
}
