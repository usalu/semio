//! 🔺 Diff constructor for `DeleteSavedCamera`.

use super::mutation::DeleteSavedCamera;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingSavedCamerasDelta, ShootingDiff};

pub fn diff(payload: &DeleteSavedCamera, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { saved_cameras: Some(ShootingSavedCamerasDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
