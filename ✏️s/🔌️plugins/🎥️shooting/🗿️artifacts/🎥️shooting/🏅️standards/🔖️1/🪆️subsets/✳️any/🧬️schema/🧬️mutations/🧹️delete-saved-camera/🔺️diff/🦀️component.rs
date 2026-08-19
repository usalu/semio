//! 🔺 Diff constructor for `DeleteSavedCamera`. Error `target-missing` when absent.

use super::mutation::DeleteSavedCamera;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingSavedCamerasDelta, ShootingDiff};

pub async fn diff(payload: &DeleteSavedCamera, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !base.saved_cameras.iter().any(|camera| camera.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Saved camera \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(ShootingDiff { saved_cameras: Some(ShootingSavedCamerasDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
