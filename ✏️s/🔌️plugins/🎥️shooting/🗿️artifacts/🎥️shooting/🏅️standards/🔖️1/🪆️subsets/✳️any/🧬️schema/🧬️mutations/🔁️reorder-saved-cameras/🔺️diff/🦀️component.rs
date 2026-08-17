//! 🔺 Diff constructor for `ReorderSavedCameras`. Error `target-missing` when absent, Warning
//! `no-op` when the resulting order is unchanged.

use super::mutation::ReorderSavedCameras;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingSavedCamerasDelta, ShootingDiff};

pub fn diff(payload: &ReorderSavedCameras, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !base.saved_cameras.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Saved camera \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let original: Vec<String> = base.saved_cameras.iter().map(|entry| entry.id.clone()).collect();
    let mut ids = original.clone();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    if ids == original {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Saved camera \"{}\" order is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(ShootingDiff { saved_cameras: Some(ShootingSavedCamerasDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() })
}
