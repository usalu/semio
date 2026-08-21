//! 🔺 Diff constructor for `DeleteShot`. Error `target-missing` when absent.

use super::mutation::DeleteShot;
use crate::artifacts::shooting::diff::{ShootingDiff, ShootingShotsDelta};
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &DeleteShot, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !base.shots.iter().any(|shot| shot.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(ShootingDiff { shots: Some(ShootingShotsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
