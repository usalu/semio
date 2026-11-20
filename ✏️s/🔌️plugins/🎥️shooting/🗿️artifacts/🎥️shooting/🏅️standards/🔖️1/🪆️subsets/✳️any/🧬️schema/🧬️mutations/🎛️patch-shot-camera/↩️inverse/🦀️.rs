//! ↩️ Restores the prior camera control and releases an unmodified first capture.

use super::{PatchShotCamera, ShotCameraSlot};
use super::component::slot_id;
use crate::ShootingSnapshot;

pub fn inverse<Op: From<PatchShotCamera>>(payload: &PatchShotCamera, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let Some(shot) = base.shots.iter().find(|shot| shot.id == payload.shot_id) else {
        return Ok(Vec::new());
    };
    let linked = shot.camera_id.as_ref().and_then(|id| base.saved_cameras.iter().find(|entry| entry.id == *id));
    let before = linked.map_or(&payload.fallback, |entry| &entry.camera).clone();
    let slot = slot_id(&shot.id);
    let release = (!linked.is_some_and(|entry| entry.id == slot)).then(|| ShotCameraSlot {
        seed: before.clone(),
        camera_id: shot.camera_id.clone(),
    });
    Ok(vec![Op::from(PatchShotCamera {
        shot_id: shot.id.clone(),
        field: payload.field.clone(),
        fallback: before.clone(),
        camera: before,
        release,
    })])
}
