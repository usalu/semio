//! 🔺 Diff constructor for `RenameSavedCamera`. Error `target-missing` when absent, Warning
//! `no-op` when already at that label.

use super::mutation::RenameSavedCamera;
use crate::artifacts::shooting::diff::{ShootingDiff, ShootingSavedCameraPatchEntry, ShootingSavedCamerasDelta};
use crate::artifacts::shooting::ShootingSavedCameraPatch;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &RenameSavedCamera, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.saved_cameras.iter().find(|camera| camera.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Saved camera \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Saved camera \"{}\" already has label \"{}\".", payload.id, payload.new_label));
    }
    protocol::MutationOutcome::new(ShootingDiff {
        saved_cameras: Some(ShootingSavedCamerasDelta { patched: vec![ShootingSavedCameraPatchEntry { id: payload.id.clone(), patch: ShootingSavedCameraPatch { label: Some(payload.new_label.clone()), camera: None } }], ..Default::default() }),
        ..Default::default()
    })
}
