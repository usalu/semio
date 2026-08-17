//! 🔺 Diff constructor for `CreateShot`. Fatal `duplicate-id` on an existing id.

use super::mutation::CreateShot;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingShotsDelta, ShootingDiff};

pub fn diff(payload: &CreateShot, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if base.shots.iter().any(|shot| shot.id == payload.shot.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A shot with id \"{}\" already exists.", payload.shot.id), [payload.shot.id.clone()]);
    }
    protocol::MutationOutcome::new(ShootingDiff { shots: Some(ShootingShotsDelta { added: vec![payload.shot.clone()], ..Default::default() }), ..Default::default() })
}
