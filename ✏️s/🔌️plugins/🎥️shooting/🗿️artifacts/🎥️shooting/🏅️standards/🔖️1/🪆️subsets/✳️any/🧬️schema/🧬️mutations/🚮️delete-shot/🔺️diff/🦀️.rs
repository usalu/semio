//! 🔺 Diff constructor for `DeleteShot`. Error `target-missing` when absent.

use super::DeleteShot;
use crate::diff::{ShootingDiff, ShootingShotsDelta};
use crate::ShootingSnapshot;

pub fn diff(payload: &DeleteShot, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !base.shots.iter().any(|shot| shot.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(ShootingDiff { shots: Some(ShootingShotsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
