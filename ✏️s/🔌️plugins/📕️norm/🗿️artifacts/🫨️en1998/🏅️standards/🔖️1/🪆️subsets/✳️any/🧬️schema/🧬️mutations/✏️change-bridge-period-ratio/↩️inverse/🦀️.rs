//! ↩️ `change-bridge-period-ratio` inverse: restores the previous `period_ratio`.

use crate::change_bridge_period_ratio::ChangeBridgePeriodRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgePeriodRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_period_ratio::ChangeBridgePeriodRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgePeriodRatio, base: &En1998Snapshot) -> Option<ChangeBridgePeriodRatio> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgePeriodRatio { id: payload.id.clone(), new_period_ratio: r0.period_ratio })
}
