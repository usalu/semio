//! ↩️ `change-bridge-track-gauge` inverse: restores the previous `track_gauge`.

use crate::change_bridge_track_gauge::ChangeBridgeTrackGauge;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeTrackGauge, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_track_gauge::ChangeBridgeTrackGauge> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeTrackGauge, base: &En1990Snapshot) -> Option<ChangeBridgeTrackGauge> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeTrackGauge { id: payload.id.clone(), new_track_gauge: r0.track_gauge })
}
