//! ↩️ `change-footbridge-assumed-service-axle` inverse: restores the previous `assumed_service_axle`.

use crate::change_footbridge_assumed_service_axle::ChangeFootbridgeAssumedServiceAxle;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFootbridgeAssumedServiceAxle, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_footbridge_assumed_service_axle::ChangeFootbridgeAssumedServiceAxle> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFootbridgeAssumedServiceAxle, base: &En1991Snapshot) -> Option<ChangeFootbridgeAssumedServiceAxle> {
    let r0 = base.footbridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFootbridgeAssumedServiceAxle { id: payload.id.clone(), new_assumed_service_axle: r0.assumed_service_axle })
}
