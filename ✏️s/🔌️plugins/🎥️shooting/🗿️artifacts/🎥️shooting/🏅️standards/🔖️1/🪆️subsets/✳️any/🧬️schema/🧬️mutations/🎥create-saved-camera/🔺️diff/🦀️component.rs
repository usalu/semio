//! 🔺 Diff constructor for `CreateSavedCamera`.

use super::mutation::CreateSavedCamera;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingSavedCamerasDelta, ShootingDiff};

pub fn diff(payload: &CreateSavedCamera, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { saved_cameras: Some(ShootingSavedCamerasDelta { added: vec![payload.saved_camera.clone()], ..Default::default() }), ..Default::default() }
}
