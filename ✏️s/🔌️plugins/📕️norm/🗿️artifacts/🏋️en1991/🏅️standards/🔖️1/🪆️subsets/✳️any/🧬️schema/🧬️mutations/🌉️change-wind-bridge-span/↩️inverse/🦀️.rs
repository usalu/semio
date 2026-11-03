//! ↩️ `change-wind-bridge-span` inverse: restores the previous `span`.

use crate::change_wind_bridge_span::ChangeWindBridgeSpan;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindBridgeSpan, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_bridge_span::ChangeWindBridgeSpan> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindBridgeSpan, base: &En1991Snapshot) -> Option<ChangeWindBridgeSpan> {
    let r0 = base.wind_bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindBridgeSpan { id: payload.id.clone(), new_span: r0.span })
}
