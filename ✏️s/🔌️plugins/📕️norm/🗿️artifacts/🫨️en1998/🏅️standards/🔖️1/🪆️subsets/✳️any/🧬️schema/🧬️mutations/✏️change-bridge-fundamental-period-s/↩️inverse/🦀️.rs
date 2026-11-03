//! ↩️ `change-bridge-fundamental-period-s` inverse: restores the previous `fundamental_period_s`.

use crate::change_bridge_fundamental_period_s::ChangeBridgeFundamentalPeriodS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeFundamentalPeriodS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_fundamental_period_s::ChangeBridgeFundamentalPeriodS> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeFundamentalPeriodS, base: &En1998Snapshot) -> Option<ChangeBridgeFundamentalPeriodS> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeFundamentalPeriodS { id: payload.id.clone(), new_fundamental_period_s: r0.fundamental_period_s })
}
