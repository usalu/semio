//! ↩ Inverse constructor for `ReplaceShotCamera` — reconstructed from BASE state.

use super::ReplaceShotCamera;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &ReplaceShotCamera, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_shot_camera::ReplaceShotCamera>,
{
    Ok((|| {
    let camera_id = match base.shots.iter().find(|shot| shot.id == payload.shot_id).and_then(|shot| shot.camera_id.clone()) {
        Some(id) => id,
        None => return Vec::new(),
    };
    match base.saved_cameras.iter().find(|entry| entry.id == camera_id) {
        Some(entry) => vec![Op::from(ReplaceShotCamera { shot_id: payload.shot_id.clone(), new_camera: entry.camera.clone() })],
        None => Vec::new(),
    }

    })())
}
