//! 🔺 Diff constructor for `ReorderSavedCameras`.

use super::mutation::ReorderSavedCameras;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingSavedCamerasDelta, ShootingDiff};

pub fn diff(payload: &ReorderSavedCameras, base: &ShootingSnapshot) -> ShootingDiff {
    let mut ids: Vec<String> = base.saved_cameras.iter().map(|entry| entry.id.clone()).collect();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    ShootingDiff { saved_cameras: Some(ShootingSavedCamerasDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() }
}
