//! ↩️ `change-bridge-line-speed` inverse: restores the previous `line_speed`.

use crate::change_bridge_line_speed::ChangeBridgeLineSpeed;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeLineSpeed, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_line_speed::ChangeBridgeLineSpeed> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeLineSpeed, base: &En1990Snapshot) -> Option<ChangeBridgeLineSpeed> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeLineSpeed { id: payload.id.clone(), new_line_speed: r0.line_speed })
}
