//! ↩️ `change-bridge-vertical-period-s` inverse: restores the previous `vertical_period_s`.

use crate::change_bridge_vertical_period_s::ChangeBridgeVerticalPeriodS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeVerticalPeriodS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_vertical_period_s::ChangeBridgeVerticalPeriodS> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeVerticalPeriodS, base: &En1998Snapshot) -> Option<ChangeBridgeVerticalPeriodS> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeVerticalPeriodS { id: payload.id.clone(), new_vertical_period_s: r0.vertical_period_s })
}
