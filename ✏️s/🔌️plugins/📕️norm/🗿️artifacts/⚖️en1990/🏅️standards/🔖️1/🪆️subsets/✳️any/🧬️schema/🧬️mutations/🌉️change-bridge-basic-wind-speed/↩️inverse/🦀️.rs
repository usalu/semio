//! ↩️ `change-bridge-basic-wind-speed` inverse: restores the previous `basic_wind_speed`.

use crate::change_bridge_basic_wind_speed::ChangeBridgeBasicWindSpeed;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeBasicWindSpeed, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_basic_wind_speed::ChangeBridgeBasicWindSpeed> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeBasicWindSpeed, base: &En1990Snapshot) -> Option<ChangeBridgeBasicWindSpeed> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeBasicWindSpeed { id: payload.id.clone(), new_basic_wind_speed: r0.basic_wind_speed })
}
