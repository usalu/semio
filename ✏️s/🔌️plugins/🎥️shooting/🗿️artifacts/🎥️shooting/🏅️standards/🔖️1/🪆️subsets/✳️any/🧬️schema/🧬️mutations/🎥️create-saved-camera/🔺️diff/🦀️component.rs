//! 🔺 Diff constructor for `CreateSavedCamera`. Fatal `duplicate-id` on an existing id.

use super::mutation::CreateSavedCamera;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingSavedCamerasDelta, ShootingDiff};

pub async fn diff(payload: &CreateSavedCamera, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if base.saved_cameras.iter().any(|camera| camera.id == payload.saved_camera.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A saved camera with id \"{}\" already exists.", payload.saved_camera.id), [payload.saved_camera.id.clone()]);
    }
    protocol::MutationOutcome::new(ShootingDiff { saved_cameras: Some(ShootingSavedCamerasDelta { added: vec![payload.saved_camera.clone()], ..Default::default() }), ..Default::default() })
}
